/// Simple rgb/rgba color structure
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    // Primary Colors
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    // Secondary Colors
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);

    /// Create a Color from RGB values
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    /// Create a Color from RGBA values
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub const fn rgba_f(r: u8, g: u8, b: u8, a: f32) -> Color {
        Color {
            r,
            g,
            b,
            a: (a.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
    /// Create a Color from a hex string like "#FF5733" or "FF5733"
    pub fn from_hex(hex: &str) -> Option<Color> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 { return None; }
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        Some(Color::rgb(r, g, b))
    }

    /// Linearly interpolate between two colors
    /// t = 0.0 returns self, t = 1.0 returns other
    pub fn lerp(self, other: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
            a: (self.a as f32 + (other.a as f32 - self.a as f32) * t) as u8,
        }
    }

    /// Returns the color with a different alpha (0.0 - 1.0)
    pub fn with_alpha(self, a: f32) -> Color {
        Color { a: (a.clamp(0.0, 1.0) * 255.0) as u8, ..self }
    }

    /// Darken the color by a factor (0.0 = black, 1.0 = unchanged)
    pub fn darken(self, factor: f32) -> Color {
        let f = factor.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 * f) as u8,
            g: (self.g as f32 * f) as u8,
            b: (self.b as f32 * f) as u8,
            a: self.a,
        }
    }

    /// Lighten the color by a factor (0.0 = unchanged, 1.0 = white)
    pub fn lighten(self, factor: f32) -> Color {
        let f = factor.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (255.0 - self.r as f32) * f) as u8,
            g: (self.g as f32 + (255.0 - self.g as f32) * f) as u8,
            b: (self.b as f32 + (255.0 - self.b as f32) * f) as u8,
            a: self.a,
        }
    }
}
