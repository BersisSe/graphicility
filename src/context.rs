use crate::{Graphics, Input};

/// Represents the data in the Frame
pub struct FrameContext {
    pub(crate) gfx: Graphics,
    pub(crate) inputs: Input,
    pub(crate) dt: f64,
}

impl FrameContext {
    pub(crate) fn new(gfx : Graphics, inputs: Input) -> Self {
        FrameContext {
            gfx,
            inputs,
            dt: 0.0
        }
    }
    /// Gets a mutable referance to the Graphics. <br>
    /// Only use this method if you only need to draw otherwise refer to the [Self::split]    .
    pub fn graphics(&mut self) -> &mut Graphics{
        &mut self.gfx
    }
    /// Returns the Delta time of the frame. Good if you are creating animations or a game. <br>
    /// It's better to call this method before `split` and `graphics` so Rust does not bother you with the Barrow Checker.
    pub fn delta_time(&self) -> f64{
        self.dt
    }
    /// Gets a referance to the Input. <br>
    /// Only use this method if you only need to read inputs otherwise refer to the [Self::split]    .
    pub fn input(&self) -> &Input{
        &self.inputs
    }
    /// Splits the Graphics and Input from the Context
    pub fn split(&mut self) -> (&mut Graphics, &Input){
        return (&mut self.gfx, &self.inputs);
    }
    
}
