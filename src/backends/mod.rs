mod pixels;

pub use pixels::PixelsBackend;
use winit::dpi::PhysicalSize;

use crate::graphics::DrawCommand;


pub trait Backend {
    fn render(&mut self, commands: &[DrawCommand]);
    fn logical_size(&self) -> (u32, u32);
    fn resize_window(&mut self, size: PhysicalSize<u32>);
}