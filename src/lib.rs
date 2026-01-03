mod backends;
mod color;
mod graphics;
mod runtime;
mod text;
mod input;
mod context;
mod config;

pub use graphics::Graphics;
pub use color::Color;
pub use config::Config;
pub use input::Input;

// Re-Exports from winit events
pub use winit::keyboard::KeyCode;
pub use winit::event::MouseButton;

use context::WindowContext;
use runtime::Runtime;

use winit::event_loop::EventLoop;



/// Run the application with default configuration
///```rust
/// run(|g|{
///   // your drawing code here
/// });
/// ```
pub fn run<F>(app: F)
where
    F: FnMut(&mut WindowContext),
{
    run_with(Config::default(), app);
}

/// Run the application with custom configuration
/// Example:
/// ```rust
/// let config = Config::builder()
///    .with_title("My App")
///    .set_window_size((1024, 768))
///    .set_logical_size((800, 600))
///    .set_resizeable(true)
///  .build();
/// run_with(config, |g|{
///   // your drawing code here
/// });
/// ```
pub fn run_with<F>(config: Config, app: F)
where
    F: FnMut(&mut WindowContext),
{
    let event_loop = EventLoop::new().unwrap();
    let mut app = Runtime::new(app, config);
    event_loop.run_app(&mut app).unwrap()
}
