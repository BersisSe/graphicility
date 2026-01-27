use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;

use crate::Config;
use crate::backends::PixelsBackend;
use crate::context::FrameContext;
use crate::graphics::Graphics;
use crate::input::Input;

#[cfg(feature = "extension")]
use crate::extensions::Extension;

/// The main runtime struct that manages the application lifecycle
/// It holds the configuration, window, graphics context, rendering backend, and the user-defined drawing function.
pub struct Runtime<F> {
    config: Config,
    window: Option<Window>,
    context: Option<FrameContext>,
    backend: Option<PixelsBackend>,
    draw_fn: F,
    last_frame_time: Instant,
    input_stepped: bool,
    #[cfg(feature = "extension")]
    extensions: Vec<Box<dyn Extension>>
}

impl<F> Runtime<F>
where
    F: FnMut(&mut FrameContext),
{
    pub(crate) fn get_input_helper(&mut self) -> &mut WinitInputHelper {
        &mut self.context.as_mut().unwrap().inputs.helper
    }
    #[cfg(not(feature = "extension"))]
    pub fn new(draw_fn: F, config: Config) -> Self {
        let logical_size = LogicalSize::new(config.logical_width, config.logical_height);
        
        let graphics = Graphics::new(
            logical_size,
            PhysicalSize::new(config.window_width, config.window_height),
        );

        let inputs = Input::new();

        Self {
            config,
            window: None,
            context: Some(FrameContext::new(graphics, inputs)),
            backend: None,
            draw_fn: draw_fn,
            last_frame_time: Instant::now(),
            input_stepped: false,
        }
    }

    #[cfg(feature = "extension")]
    pub fn new(draw_fn: F, mut config: Config) -> Self {
        let logical_size = LogicalSize::new(config.logical_width, config.logical_height);
        // Move the extensions out of the config early
        
        let mut extensions = std::mem::take(&mut config.extensions); // Take the extensions out of config
        extensions.iter_mut().for_each(|ext|ext.on_init());
        let graphics = Graphics::new(
            logical_size,
            PhysicalSize::new(config.window_width, config.window_height),
        );

        let inputs = Input::new();

        Self {
            config,
            window: None,
            context: Some(FrameContext::new(graphics, inputs)),
            backend: None,
            draw_fn: draw_fn,
            last_frame_time: Instant::now(),
            input_stepped: false,
            #[cfg(feature = "extension")]
            extensions: extensions
        }
    }
}

impl<F> ApplicationHandler for Runtime<F>
where
    F: FnMut(&mut FrameContext),
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let config = &self.config;

        let attrs = WindowAttributes::default()
            .with_title(&config.title)
            .with_resizable(config.resizeable)
            .with_inner_size(PhysicalSize::new(config.window_width, config.window_height));

        let window = event_loop.create_window(attrs).unwrap();

        let physical_size = window.inner_size();
        let logical_size = LogicalSize::new(config.logical_width, config.logical_height);

        self.backend = Some(PixelsBackend::new(&window, physical_size, logical_size));
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Process keyboard events
        if let WindowEvent::KeyboardInput { event, .. } = &event {
            self.context
                .as_mut()
                .unwrap()
                .inputs
                .process_key_event(event);
        }

        if self.get_input_helper().process_window_event(&event) {
            let context = self.context.as_mut().unwrap();
            let renderer = self.backend.as_mut().unwrap();
            renderer.render(context.gfx.commands())
        }
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                    return;
                }

                if let Some(renderer) = &mut self.backend {
                    renderer.resize_window(physical_size);
                    // Sync new logical size to graphics context
                    let (lw, lh) = renderer.logical_size();
                    let ctx = self.context.as_mut().unwrap();
                    ctx.gfx.set_logical_size(lw, lh);
                }

                let ctx = self.context.as_mut().unwrap();
                ctx.gfx.window_width = physical_size.width;
                ctx.gfx.window_height = physical_size.height;
            }

            _ => (),
        }
    }

    fn device_event(
        &mut self,
        _: &ActiveEventLoop,
        _: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.get_input_helper().process_device_event(&event);
    }

    // Mark frame start for input handling
    fn new_events(&mut self, _: &ActiveEventLoop, _: winit::event::StartCause) {
        if !self.input_stepped {
            self.get_input_helper().step();
            self.input_stepped = true;
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(win) = &self.window {
            let elapsed = self.last_frame_time.elapsed();

            // Calculate frame timing
            let should_run = if let Some(target_fps) = self.config.target_fps {
                elapsed >= Duration::from_secs_f64(1.0 / target_fps as f64)
            } else {
                true
            };

            if should_run {
                let mut context = self.context.as_mut().unwrap();
                context.dt = elapsed.as_secs_f64().min(0.1);
                self.last_frame_time = Instant::now();

                context.inputs.update_mouse_mapping(&context.gfx);
                context.gfx.begin_frame();
                

                #[cfg(feature = "extension")]
                for ext in &mut self.extensions{
                    ext.pre_draw(&mut context);
                }

                (self.draw_fn)(&mut context);

                #[cfg(feature = "extension")]
                for ext in &mut self.extensions{
                    ext.post_draw(&mut context);
                }
                context.inputs.helper.end_step();
                context.inputs.reset_transient_state();

                // Flag Reset for the new frame
                self.input_stepped = false;

                win.request_redraw();
            } else if let Some(target_fps) = self.config.target_fps {
                let frame_duration = Duration::from_secs_f64(1.0 / target_fps as f64);
                event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
                    Instant::now() + (frame_duration - elapsed),
                ));
            }
        }
    }
}
