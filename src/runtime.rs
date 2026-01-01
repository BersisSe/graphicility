use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;

use winit::window::{Window, WindowAttributes, WindowId};

use crate::Config;
use crate::backends::PixelsBackend;
use crate::graphics::Graphics;

/// The main runtime struct that manages the application lifecycle
/// It holds the configuration, window, graphics context, rendering backend, and the user-defined drawing function.
pub struct Runtime<F> {
    config: Config,
    window: Option<Window>,
    graphics: Option<Graphics>,
    backend: Option<PixelsBackend>,
    draw_fn: F,
}

impl<F> Runtime<F>
where
    F: FnMut(&mut Graphics),
{
    pub fn new(draw_fn: F, config: Config) -> Self {
        Self {
            config,
            window: None,
            graphics: None,
            backend: None,
            draw_fn: draw_fn,
        }
    }
}

impl<F> ApplicationHandler for Runtime<F>
where
    F: FnMut(&mut Graphics),
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
        self.graphics = Some(Graphics::new(logical_size, physical_size));
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let mut graphics = self.graphics.as_mut().unwrap();
                let renderer = self.backend.as_mut().unwrap();
                graphics.begin_frame();

                (self.draw_fn)(&mut graphics);

                renderer.render(graphics.commands());
            }
            WindowEvent::Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                    return;
                }
                if let Some(renderer) = &mut self.backend {
                    renderer.resize_window(physical_size);
                }
            }

            _ => (),
        }
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(win) = &self.window {
            win.request_redraw();
        }
    }
}
