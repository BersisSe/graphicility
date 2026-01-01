use crate::{Color, graphics::DrawCommand};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};

pub struct PixelsBackend {
    pixels: Pixels,
    logic_width: u32,
    logic_height: u32,
}

impl PixelsBackend {
    pub(crate) fn new(
        window: &winit::window::Window,
        window_size: PhysicalSize<u32>,
        logic_size: LogicalSize<u32>,
    ) -> Self {
        // Create a surface texture that maps the logical buffer to the physical window
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);

        let pixels: Pixels =
            PixelsBuilder::new(logic_size.width, logic_size.height, surface_texture)
                .enable_vsync(true)
                .build()
                .expect("Error While Creating Pixels");

        Self {
            pixels,
            logic_height: logic_size.height,
            logic_width: logic_size.width,
        }
    }
    fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Color) {
        let mut cursor_x = x;
        let cursor_y = y;

        for c in text.chars() {
            let char_code = c as usize;

            // Using FONT8X8_BASIC covers ASCII 0-127
            if char_code >= crate::text::FONT8X8_BASIC.len() {
                cursor_x += 8;
                continue;
            }

            let glyph = &crate::text::FONT8X8_BASIC[char_code];

            // Draw each row of the 8x8 character
            for row in 0..8 {
                let byte = glyph[row];

                // Draw each pixel in the row
                for col in 0..8 {
                    // Check if this bit is set
                    if (byte & (1 << col)) != 0 {
                        let px = cursor_x + col;
                        let py = cursor_y + row as i32;

                        // Only draw if within bounds
                        if px >= 0
                            && py >= 0
                            && (px as u32) < self.logic_width
                            && (py as u32) < self.logic_height
                        {
                            self.set_pixel(px as u32, py as u32, color);
                        }
                    }
                }
            }

            // Move cursor to next character position
            cursor_x += 8;
        }
    }

    
    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: Color) {
        // 1. Calculate boundaries using saturating math to prevent overflow panics
        let x1 = x;
        let y1 = y;
        let x2 = x.saturating_add(w);
        let y2 = y.saturating_add(h);

        // 2. Clip to the logical screen size
        let start_x = x1.min(self.logic_width);
        let start_y = y1.min(self.logic_height);
        let end_x = x2.min(self.logic_width);
        let end_y = y2.min(self.logic_height);

        // If the rect is entirely off-screen, start will equal end
        if start_x >= end_x || start_y >= end_y {
            return;
        }

        let frame = self.pixels.frame_mut();
        let color_slice = [color.r, color.g, color.b, color.a];

        for row in start_y..end_y {
            let offset = (row * self.logic_width + start_x) as usize * 4;
            let row_pixels = (end_x - start_x) as usize;

            let target_row = &mut frame[offset..offset + (row_pixels * 4)];
            for px in target_row.chunks_exact_mut(4) {
                px.copy_from_slice(&color_slice);
            }
        }
    }
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let mut x = x0;
        let mut y = y0;

        loop {
            if x >= 0 && y >= 0 {
                self.set_pixel(x as u32, y as u32, color);
            }
            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x += sx; }
            if e2 <= dx { err += dx; y += sy; }
        }
    }
    fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        self.draw_line(x1, y1, x2, y2, color);
        self.draw_line(x2, y2, x3, y3, color);
        self.draw_line(x3, y3, x1, y1, color);
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.logic_width || y >= self.logic_height {
            return;
        }

        let idx = ((y * self.logic_width + x) * 4) as usize;
        let frame = self.pixels.frame_mut();
        frame[idx..idx + 4].copy_from_slice(&[color.r, color.g, color.b, color.a]);
    }

    fn clear(&mut self, color: Color) {
        let frame = self.pixels.frame_mut();
        let color_slice = [color.r, color.g, color.b, color.a];
        for px in frame.chunks_exact_mut(4) {
            px.copy_from_slice(&color_slice);
        }
    }

    pub fn resize_window(&mut self, size: PhysicalSize<u32>) {
        if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
            eprintln!("Pixels resize_surface failed: {}", err);
        }
    }

    pub fn render(&mut self, commands: &[DrawCommand]) {
        for cmd in commands {
            match cmd {
                DrawCommand::Clear(color) => self.clear(*color),
                DrawCommand::Pixel { x, y, color } => self.set_pixel(*x, *y, *color),
                DrawCommand::Rect { x, y, w, h, color } => self.draw_rect(*x, *y, *w, *h, *color),
                DrawCommand::Text { x, y, text, color } => self.draw_text(*x, *y, text, *color),
                DrawCommand::Line { x0, y0, x1, y1, color } => self.draw_line(*x0, *y0, *x1, *y1, *color),
                DrawCommand::Triangle { x1, y1, x2, y2, x3, y3, color } => self.draw_triangle(*x1, *y1, *x2, *y2, *x3, *y3, *color)
            }
        }

        if let Err(err) = self.pixels.render() {
            eprintln!("Pixels render failed: {}", err);
        }
    }
}
