use winit::dpi::{LogicalSize, PhysicalSize};

use crate::Color;
use crate::vector::Vec2;

pub enum DrawCommand {
    Clear(Color),
    Pixel {
        pos: Vec2,
        color: Color,
    },
    Line {
        start: Vec2,
        end: Vec2,
        color: Color,
    },
    Rect {
        pos: Vec2,
        size: Vec2,
        color: Color,
    },
    Circle{
        center : Vec2,
        radius: i32,
        color: Color,
    },
    Triangle {
        p1: Vec2,
        p2: Vec2,
        p3: Vec2,
        color: Color,
    },
    Text {
        pos: Vec2,
        text: String,
        color: Color,
    },
}

/// This is the Main Drawing interface.
pub struct Graphics {
    commands: Vec<DrawCommand>,
    logic_width: u32,
    logic_height: u32,

    pub(crate) window_width: u32,
    pub(crate) window_height: u32,
}

impl Graphics {
    /// Create a new Graphics Struct
    pub(crate) fn new(logic_size: LogicalSize<u32>, phy_size: PhysicalSize<u32>) -> Self {
        Graphics {
            commands: Vec::with_capacity(128),
            logic_height: logic_size.height,
            logic_width: logic_size.width,
            window_height: phy_size.height,
            window_width: phy_size.width,
        }
    }
    /// Clear commands at the start of each frame
    /// This should be called internally by the runtime at the before rendering
    pub(crate) fn begin_frame(&mut self) {
        self.commands.clear();
    }
    /// Get the list of draw commands for this frame
    pub(crate) fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }
    /// Get the logical size of the window
    pub fn logical_size(&self) -> (u32, u32) {
        (self.logic_width, self.logic_height)
    }
    /// Get the physical window size
    pub fn window_size(&self) -> (u32, u32) {
        (self.window_width, self.window_height)
    }
    /// Clear the screen with a color  
    /// You should call this at the start of each frame to clear the previous frame's drawings
    pub fn clear(&mut self, color: Color) {
        self.commands.push(DrawCommand::Clear(color));
    }
    /// Draw a single pixel to a given Point as a [Vec2]
    pub fn pixel(&mut self, pos: impl Into<Vec2>, color: Color) {
        let p = pos.into(
            
        );
        // Internal guard check
        if p.x < 0 || p.y < 0 { return; }

        self.commands.push(DrawCommand::Pixel { pos: p, color });
    }
    /// Draw a line between 2 Points.
    pub fn line(&mut self, start: impl Into<Vec2>, end: impl Into<Vec2>, color: Color) {
        self.commands.push(DrawCommand::Line {
            start: start.into(),
            end: end.into(),
            color,
        });
    }
    /// Draw a filled Rectangle on `pos` with a given `size`.
    pub fn rect(&mut self, pos: impl Into<Vec2>, size: impl Into<Vec2>, color: Color) {
        let p = pos.into();
        let s = size.into();
        
        // We can still do your negative clipping logic here easily
        if s.x <= 0 || s.y <= 0 { return; }

        self.commands.push(DrawCommand::Rect { pos: p, size: s, color });
    }
    pub fn circle(&mut self, center: impl Into<Vec2>, radius: i32, color : Color){
        self.commands.push(DrawCommand::Circle { center: center.into(), radius,  color});
    }
    /// Draw a Hollow Triangle using the given 3 points.
    pub fn triangle(&mut self, p1: impl Into<Vec2>, p2: impl Into<Vec2>, p3: impl Into<Vec2>, color: Color) {
        self.commands.push(DrawCommand::Triangle {
            p1: p1.into(),
            p2: p2.into(),
            p3: p3.into(),
            color,
        });
    }
    /// Draw text at `pos` with the specified color using the internal 8x8 bitmap font
    pub fn text<T: Into<String>>(&mut self, pos: impl Into<Vec2>, text: T, color: Color) {
        self.commands.push(DrawCommand::Text {
            pos: pos.into(),
            text: text.into(),
            color,
        });
    }   
}
