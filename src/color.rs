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
    pub const YELLOW: Color = Color::rgb(255, 255, 0,);
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    

    /// Create a Color from RGB values
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color{
        Color { r, g, b, a: 255 }
    }
    /// Create a Color from RGBA values
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color{
        // turn the 0.0-1.0 alpha into 0-255 u8
        let a = (a.clamp(0.0, 1.0) * 255.0) as u8;
        Color { r, g, b,  a}
    }
}
