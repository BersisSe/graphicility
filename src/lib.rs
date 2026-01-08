//! # Graphiclity
//!
//! A minimal, immediate-mode 2D drawing library designed for simplicity and ease of use.
//!
//! Graphiclity provides a higher-level interface around `pixels` and `winit`, offering a
//! logical pixel buffer that automatically handles scaling and DPI. <br>
//! Allowing you to create simple graphical applications.
//!
//! ## Core Concepts
//!
//! - **[FrameContext]**: The primary interface provided to your draw loop. It contains
//!   access to [Graphics], [Input], and timing data.
//! - **Logical Resolution**: You define a fixed "Virtual resolution" (e.g., 320x240). 
//!   The library scales this to fit the physical window.
//! - **[Vec2]**: A flexible coordinate type. Most methods accept `impl Into<Vec2>`,
//!   allowing you to pass `(x, y)` tuples directly.
//! - **Immediate Mode** : You define the graphics and you see it _immediately_
//! ## Some Examples
#![doc = include_str!("docs/examples.md")]

mod backends;
mod color;
mod graphics;
mod runtime;
mod text;
mod input;
mod context;
mod config;
mod vector;

pub use graphics::Graphics;
pub use context::FrameContext;
pub use color::Color;
pub use config::Config;
pub use input::Input;
pub use vector::Vec2;

// Re-Exports from winit events
pub use winit::keyboard::KeyCode;
pub use winit::event::MouseButton;

use runtime::Runtime;

use winit::event_loop::EventLoop;



/// Run the application with default configuration
///```rust
/// run(|ctx|{
///   // your drawing code here
/// });
/// ```
pub fn run<F>(app: F)
where
    F: FnMut(&mut FrameContext),
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
    F: FnMut(&mut FrameContext),
{
    let event_loop = EventLoop::new().unwrap();
    let mut app = Runtime::new(app, config);
    event_loop.run_app(&mut app).unwrap()
}
