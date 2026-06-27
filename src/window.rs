use std::error::Error;

use winit::event_loop::EventLoop;
use winit::platform::pump_events::EventLoopExtPumpEvents;

use crate::Config;
use crate::context::FrameContext;
use crate::runtime::Runtime;

/// A handle to the application window and render loop.
///
/// `Window` gives you full control over the frame loop, making it ideal
/// for emulators, simulators, or anything that needs to tick independently
/// from the render cycle.
///
/// # Example
///
/// ```rust
/// use graphicility::{Window, Config, Color};
///
/// let mut window = Window::new(Config::default());
///
/// while window.is_running() {
///     let ctx = window.begin_frame();
///     ctx.graphics().clear(Color::BLACK);
///     ctx.graphics().rect((10, 10), (50, 50), Color::RED);
///     window.end_frame();
/// }
/// ```
pub struct Window {
    runtime: Runtime,
    event_loop: EventLoop<()>,
}

impl Window {
    /// Create a new window with the given configuration.
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let event_loop = EventLoop::new()?;
        let runtime = Runtime::new(config);
        Ok(Window {
            runtime,
            event_loop,
        })
    }

    /// Returns `true` if the window is still open and running.
    pub fn is_running(&self) -> bool {
        self.runtime.is_running()
    }

    /// Begin a new frame. Call your drawing code on the returned [`FrameContext`].
    ///
    /// Returns `None` if the frame should be skipped (e.g. FPS cap not reached yet).
    /// Calling unwrap on this method would probably result in a crash cause the window is not initialized.
    pub fn begin_frame(&mut self) -> Option<&mut FrameContext> {
        // Pump the event loop once to process window/input events
        self.event_loop
            .pump_app_events(Some(std::time::Duration::ZERO), &mut self.runtime);

        self.runtime.begin_frame()
    }
    /// Block until the next frame is ready and return it.
    ///
    /// This is the simplest way to drive the loop — it handles FPS capping
    /// and initialization internally, returning `None` only when the window closes.
    ///
    /// # Example
    /// ```rust
    /// while let Some(frame) = win.next_frame() {
    ///     frame.graphics().clear(Color::BLACK);
    ///     win.end_frame();
    /// }
    /// ```
    pub fn next_frame(&mut self) -> &mut FrameContext {
    loop {
        if self.begin_frame().is_some() {
            return self.runtime.context.as_mut().unwrap();
        }
    }
}

    /// End the current frame and present it to the screen.
    pub fn end_frame(&mut self) {
        self.runtime.end_frame();
    }
}
