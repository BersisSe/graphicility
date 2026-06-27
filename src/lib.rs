//! # Graphicility
//!
//! A minimal, immediate-mode 2D drawing library designed for simplicity and ease of use.
//!
//! Graphicility provides a higher-level interface around `pixels` and `winit`, offering a
//! logical pixel buffer that automatically handles scaling and DPI. <br>
//! Allowing you to create simple graphical applications.
//!
//! ## Core Concepts
//!
//! - **[Window]**: The new primary entry point. Gives you full control over the frame loop.
//! - **[FrameContext]**: The interface provided each frame. Contains [Graphics], [Input], and timing.
//! - **Logical Resolution**: You define a fixed "virtual resolution" (e.g., 320x240).
//!   The library scales this to fit the physical window.
//! - **[Vec2]**: A flexible coordinate type. Most methods accept `impl Into<Vec2>`,
//!   allowing you to pass `(x, y)` tuples directly.
//! - **Immediate Mode**: You define the graphics and you see it _immediately_.
//!
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
mod window;

#[cfg(feature = "extension")]
pub mod extensions;

use std::error::Error;

pub use graphics::Graphics;
pub use context::FrameContext;
pub use color::Color;
pub use config::Config;
pub use input::Input;
pub use vector::{Vec2, Rect};
pub use window::Window;

// Re-exports from winit
pub use winit::keyboard::KeyCode;
pub use winit::event::MouseButton;



/// Run the application with default configuration.
///
/// This is the simplest way to get started. For more control over the loop,
/// use [`Window`] directly.
///
/// ```rust
/// use graphicility::{run, Color};
///
/// run(|ctx| {
///     ctx.graphics().clear(Color::WHITE);
/// });
/// ```
pub fn run<F>(draw_fn: F) -> Result<(), Box<dyn Error>>
where
    F: FnMut(&mut FrameContext),
{
    run_with(Config::default(), draw_fn)
}

/// Run the application with custom configuration.
///
/// For more control over the loop, use [`Window`] directly.
///
/// ```rust
/// use graphicility::{run_with, Config, Color};
///
/// let config = Config::builder()
///     .with_title("My App")
///     .set_window_size((1024, 768))
///     .set_logical_size((800, 600))
///     .set_resizeable(true)
///     .build();
///
/// run_with(config, |ctx| {
///     ctx.graphics().clear(Color::WHITE);
/// });
/// ```
pub fn run_with<F>(config: Config, mut draw_fn: F) -> Result<(), Box<dyn Error>>
where
    F: FnMut(&mut FrameContext),
{
    let mut window = Window::new(config)?;

    while window.is_running() {
        if let Some(ctx) = window.begin_frame() {
            draw_fn(ctx);
        }
        window.end_frame();
    }

    Ok(())
}