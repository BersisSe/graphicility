use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};
use winit_input_helper::WinitInputHelper;

use crate::Config;
use crate::backends::{Backend, PixelsBackend};
use crate::context::FrameContext;
use crate::graphics::Graphics;
use crate::input::Input;

#[cfg(feature = "extension")]
use crate::extensions::Extension;

/// Internal state machine for the window and rendering pipeline.
/// This no longer holds the draw function — that's the caller's responsibility.
pub struct Runtime<B: Backend = PixelsBackend> {
    pub(crate) config: Config,
    pub(crate) window: Option<winit::window::Window>,
    pub(crate) context: Option<FrameContext>,
    pub(crate) backend: Option<B>,
    pub(crate) last_frame_time: Instant,
    pub(crate) input_stepped: bool,
    pub(crate) running: bool,
    #[cfg(feature = "extension")]
    pub(crate) extensions: Vec<Box<dyn Extension>>,
}

impl<B: Backend> Runtime<B> {
    pub(crate) fn get_input_helper(&mut self) -> &mut WinitInputHelper {
        &mut self.context.as_mut().unwrap().inputs.helper
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Call this at the start of every frame.
    /// Returns None if it's not yet time to render (e.g. FPS cap not reached).
    pub fn begin_frame(&mut self) -> Option<&mut FrameContext> {
        let elapsed = self.last_frame_time.elapsed();

        let should_run = if let Some(target_fps) = self.config.target_fps {
            elapsed >= Duration::from_secs_f64(1.0 / target_fps as f64)
        } else {
            true
        };

        if !should_run {
            return None;
        }

        let context = self.context.as_mut().unwrap();
        context.dt = elapsed.as_secs_f64().min(0.1);
        self.last_frame_time = Instant::now();

        context.inputs.update_mouse_mapping(&context.gfx);
        context.gfx.begin_frame();

        #[cfg(feature = "extension")]
        for ext in &mut self.extensions {
            ext.pre_draw(context);
        }

        Some(self.context.as_mut().unwrap())
    }

    /// Call this at the end of every frame after drawing.
    pub fn end_frame(&mut self) {
        let context = self.context.as_mut().unwrap();

        #[cfg(feature = "extension")]
        for ext in &mut self.extensions {
            ext.post_draw(context);
        }

        context.inputs.helper.end_step();
        context.inputs.reset_transient_state();
        self.input_stepped = false;

        if let Some(win) = &self.window {
            win.request_redraw();
        }
    }
}

#[cfg(not(feature = "extension"))]
impl Runtime {
    pub fn new(config: Config) -> Self {
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
            last_frame_time: Instant::now(),
            input_stepped: false,
            running: true,
        }
    }
}

#[cfg(feature = "extension")]
impl Runtime {
    pub fn new(mut config: Config) -> Self {
        let logical_size = LogicalSize::new(config.logical_width, config.logical_height);

        let mut extensions = std::mem::take(&mut config.extensions);
        extensions.iter_mut().for_each(|ext| ext.on_init());

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
            last_frame_time: Instant::now(),
            input_stepped: false,
            running: true,
            extensions,
        }
    }
}

impl ApplicationHandler for Runtime {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let config = &self.config;

        let attrs = WindowAttributes::default()
            .with_title(&config.title)
            .with_resizable(config.resizeable)
            .with_inner_size(PhysicalSize::new(config.window_width, config.window_height));

        let window = event_loop.create_window(attrs).unwrap();

        let physical_size = window.inner_size();
        let logical_size = LogicalSize::new(config.logical_width, config.logical_height);

        self.backend = Some(PixelsBackend::new(
            &window,
            physical_size,
            logical_size,
            config.letterboxing,
        ));
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
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
            renderer.render(context.gfx.commands());
        }

        match event {
            WindowEvent::CloseRequested => {
                self.running = false;
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                    return;
                }

                if let Some(renderer) = &mut self.backend {
                    renderer.resize_window(physical_size);
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

    fn new_events(&mut self, _: &ActiveEventLoop, _: winit::event::StartCause) {
        if !self.input_stepped {
            self.get_input_helper().step();
            self.input_stepped = true;
        }
    }

    // about_to_wait is now a no-op — the caller drives the frame via begin/end_frame
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {}
}