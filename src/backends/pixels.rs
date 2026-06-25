use crate::{Color, backends::Backend, graphics::DrawCommand, vector::Vec2};
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};

pub struct PixelsBackend {
    pixels: Pixels,
    logic_width: u32,
    logic_height: u32,
    use_letterboxing: bool,
}

impl PixelsBackend {
    pub(crate) fn new(
        window: &winit::window::Window,
        window_size: PhysicalSize<u32>,
        logic_size: LogicalSize<u32>,
        use_letterboxing: bool,
    ) -> Self {
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
            use_letterboxing,
        }
    }
    fn draw_text(&mut self, pos: Vec2, text: &str, color: Color) {
        let (x, y) = pos.as_u32_tuple();
        let mut cursor_x = x;
        let cursor_y = y;

        for c in text.chars() {
            let char_code = c as usize;

            // Using FONT8X8_BASIC covers ASCII 0-127.
            if char_code >= crate::text::FONT8X8_BASIC.len() {
                cursor_x += 8;
                continue;
            }

            let glyph = &crate::text::FONT8X8_BASIC[char_code];

            // Draw each row of the 8x8 character.
            for row in 0..8 {
                let byte = glyph[row];

                // Draw each pixel in the row.
                for col in 0..8 {
                    // Check if this bit is set.
                    if (byte & (1 << col)) != 0 {
                        let px = cursor_x + col;
                        let py = cursor_y + row as u32;

                        // Only draw if within bounds.
                        if (px as u32) < self.logic_width && (py as u32) < self.logic_height {
                            self.set_pixel(px as u32, py as u32, color);
                        }
                    }
                }
            }

            // Move cursor to next character position
            cursor_x += 8;
        }
    }

    fn draw_rect(&mut self, pos: Vec2, size: Vec2, color: Color) {
        let (x, y) = pos.as_u32_tuple();
        let (w, h) = size.as_u32_tuple();

        let x1 = x;
        let y1 = y;
        let x2 = x.saturating_add(w);
        let y2 = y.saturating_add(h);

        let start_x = x1.min(self.logic_width);
        let start_y = y1.min(self.logic_height);
        let end_x = x2.min(self.logic_width);
        let end_y = y2.min(self.logic_height);

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
    fn draw_rect_lines(&mut self, pos: Vec2, size: Vec2, color: Color) {
        let x = pos.x;
        let y = pos.y;
        let w = size.x;
        let h = size.y;
        // Using the new cool draw line btw😎
        self.draw_line(Vec2::new(x, y), Vec2::new(x + w, y), color);
        self.draw_line(Vec2::new(x, y + h), Vec2::new(x + w, y + h), color);
        self.draw_line(Vec2::new(x, y), Vec2::new(x, y + h), color);
        self.draw_line(Vec2::new(x + w, y), Vec2::new(x + w, y + h), color);
    }
    // Do you know how long this took to implement 😭
    fn draw_line(&mut self, p1: Vec2, p2: Vec2, color: Color) {
        let mut x0 = p1.x;
        let mut y0 = p1.y;
        let x1 = p2.x;
        let y1 = p2.y;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0
                && y0 >= 0
                && (x0 as u32) < self.logic_width
                && (y0 as u32) < self.logic_height
            {
                self.set_pixel(x0 as u32, y0 as u32, color);
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
    fn draw_circle(&mut self, center: Vec2, radius: i32, color: Color) {
        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            let pts = [
                (center.x + x, center.y + y),
                (center.x + y, center.y + x),
                (center.x - y, center.y + x),
                (center.x - x, center.y + y),
                (center.x - x, center.y - y),
                (center.x - y, center.y - x),
                (center.x + y, center.y - x),
                (center.x + x, center.y - y),
            ];
            for (px, py) in pts {
                if px >= 0 && py >= 0 {
                    self.set_pixel(px as u32, py as u32, color);
                }
            }
            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }
    fn draw_circle_filled(&mut self, center: Vec2, radius: i32, color: Color) {
        let cx = center.x;
        let cy = center.y;

        for y in -radius..=radius {
            let x_span = ((radius * radius - y * y) as f32).sqrt() as i32;

            let x_start = cx - x_span;
            let x_end = cx + x_span;
            let py = cy + y;

            if py < 0 || py as u32 >= self.logic_height {
                continue;
            }

            for x in x_start..=x_end {
                if x >= 0 && (x as u32) < self.logic_width {
                    self.set_pixel(x as u32, py as u32, color);
                }
            }
        }
    }
    fn draw_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p3, color);
        self.draw_line(p3, p1, color);
    }
    fn draw_triangle_filled(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        // Sort points by Y
        let mut pts = [p1, p2, p3];
        pts.sort_by_key(|p| p.y);
        let [a, b, c] = pts;

        for y in a.y..=c.y {
            let mut x_start = i32::MAX;
            let mut x_end = i32::MIN;

            let edges = [(a, b), (a, c), (b, c)];
            for (p, q) in edges {
                if (p.y..=q.y).contains(&y) && q.y != p.y {
                    let t = (y - p.y) as f32 / (q.y - p.y) as f32;
                    let x = (p.x as f32 + t * (q.x - p.x) as f32) as i32;
                    x_start = x_start.min(x);
                    x_end = x_end.max(x);
                }
            }

            for x in x_start..=x_end {
                if x >= 0 && y >= 0 {
                    self.set_pixel(x as u32, y as u32, color);
                }
            }
        }
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
}

impl Backend for PixelsBackend {
    fn render(&mut self, commands: &[DrawCommand]) {
        for cmd in commands {
            match cmd {
                DrawCommand::Clear(color) => self.clear(*color),
                DrawCommand::Pixel { pos, color } => {
                    let (x, y) = pos.as_u32_tuple();
                    self.set_pixel(x, y, *color);
                }
                DrawCommand::DrawBlit {
                    pos,
                    width,
                    height,
                    pixels,
                } => {
                    let frame = self.pixels.frame_mut();
                    

                    for y in 0..*height {
                        let dst_y = pos.y as u32 + y;
                        if dst_y >= self.logic_height {
                            break;
                        }

                        let row_offset = (dst_y * self.logic_width) as usize;

                        for x in 0..*width {
                            let dst_x = pos.x as u32 + x;
                            if dst_x >= self.logic_width {
                                break;
                            }

                            let src = pixels[(y * width + x) as usize];
                            let dst = (row_offset + dst_x as usize) * 4;
                            frame[dst] = src.r;
                            frame[dst + 1] = src.g;
                            frame[dst + 2] = src.b;
                            frame[dst + 3] = src.a;
                        }
                    }
                }
                DrawCommand::Circle {
                    center,
                    radius,
                    color,
                    filled,
                } => {
                    if *filled {
                        self.draw_circle_filled(*center, *radius, *color)
                    } else {
                        self.draw_circle(*center, *radius, *color)
                    }
                }
                DrawCommand::Rect {
                    pos,
                    size,
                    color,
                    filled,
                } => {
                    if *filled {
                        self.draw_rect(*pos, *size, *color)
                    } else {
                        self.draw_rect_lines(*pos, *size, *color)
                    }
                }
                DrawCommand::Text { pos, text, color } => self.draw_text(*pos, text, *color),
                DrawCommand::Line { start, end, color } => self.draw_line(*start, *end, *color),
                DrawCommand::Triangle {
                    p1,
                    p2,
                    p3,
                    color,
                    filled,
                } => {
                    if *filled {
                        self.draw_triangle_filled(*p1, *p2, *p3, *color)
                    } else {
                        self.draw_triangle(*p1, *p2, *p3, *color)
                    }
                }
            }
        }
        if let Err(err) = self.pixels.render() {
            eprintln!("Pixels render failed: {}", err);
        }
    }

    fn logical_size(&self) -> (u32, u32) {
        (self.logic_width, self.logic_height)
    }

    fn resize_window(&mut self, size: PhysicalSize<u32>) {
        if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
            eprintln!("Pixels resize_surface failed: {}", err);
        }

        if !self.use_letterboxing {
            // Also resize the buffer to match window aspect ratio (eliminates letterboxing)
            let new_logical_width = size.width / 2;
            let new_logical_height = size.height / 2;
            if new_logical_width > 0 && new_logical_height > 0 {
                if let Err(err) = self
                    .pixels
                    .resize_buffer(new_logical_width, new_logical_height)
                {
                    eprintln!("Pixels resize_buffer failed: {}", err);
                }
                self.logic_width = new_logical_width;
                self.logic_height = new_logical_height;
            }
        }
    }
}
