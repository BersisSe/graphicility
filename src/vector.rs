
/// Vec2 Represents a 2 Dimentional point. <br>
/// (i32,i32) and (f32,f32) Tuples could be converted to `Vec2` by using `into`.  <br>
/// _Note : When using `into` with floating point numbers it gets casted into i32(using .floor) and loses precision_
#[derive(Clone, Copy, Debug)]
pub struct Vec2{
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    /// Contruct a new Vec2 from X & Y cordinates
    pub fn new(x: i32, y: i32) -> Vec2{
        Vec2 { x, y }
    }
    pub fn as_u32_tuple(&self) -> (u32, u32) {
        (self.x.max(0) as u32, self.y.max(0) as u32)
    }
}

impl Into<Vec2> for (i32,i32) {
    fn into(self) -> Vec2 {
        Vec2 { x: self.0, y: self.1 }
    }
}

impl Into<Vec2> for (u32,u32) {
    fn into(self) -> Vec2 {
        Vec2 { x: self.0 as i32, y: self.1 as i32 }
    }
}

impl Into<Vec2> for (f32,f32) {
    fn into(self) -> Vec2 {
        Vec2 { 
            x: self.0.floor() as i32,

            y: self.1.floor() as i32, 
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2 { x: self.x - other.x, y: self.y - other.y }
    }
}
impl std::ops::Div for Vec2 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec2 { x: self.x / other.x, y: self.y / other.y }
    }
}
impl std::ops::Mul for Vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec2 { x: self.x * other.x, y: self.y * other.y }
    }
}
impl std::ops::Mul<i32> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: i32) -> Self {
        Vec2 { x: self.x * scalar, y: self.y * scalar }
    }
}

impl std::ops::Div<i32> for Vec2 {
    type Output = Self;
    fn div(self, scalar: i32) -> Self {
        Vec2 { x: self.x / scalar, y: self.y / scalar }
    }
}
/// A Typed Rectangle. Can be used for bounds checking
pub struct Rect{
    pub pos: Vec2,
    pub size: Vec2,
}
impl Rect {
    pub fn new(pos: Vec2, size: Vec2) -> Self {
        Self { pos, size }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.pos.x &&
        point.x < self.pos.x + self.size.x &&
        point.y >= self.pos.y &&
        point.y < self.pos.y + self.size.y
    }

    /// Returns the center point of the rectangle
    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.pos.x + self.size.x / 2,
            self.pos.y + self.size.y / 2,
        )
    }

    /// Returns a new Rect moved by an offset
    pub fn translate(&self, offset: Vec2) -> Rect {
        Rect::new(self.pos + offset, self.size)
    }

    /// Returns true if this rect overlaps with another
    pub fn intersects(&self, other: &Rect) -> bool {
        self.pos.x < other.pos.x + other.size.x &&
        self.pos.x + self.size.x > other.pos.x &&
        self.pos.y < other.pos.y + other.size.y &&
        self.pos.y + self.size.y > other.pos.y
    }

    /// Returns the overlapping region of two rects, if any
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x1 = self.pos.x.max(other.pos.x);
        let y1 = self.pos.y.max(other.pos.y);
        let x2 = (self.pos.x + self.size.x).min(other.pos.x + other.size.x);
        let y2 = (self.pos.y + self.size.y).min(other.pos.y + other.size.y);

        if x2 > x1 && y2 > y1 {
            Some(Rect::new(Vec2::new(x1, y1), Vec2::new(x2 - x1, y2 - y1)))
        } else {
            None
        }
    }

    /// Shrinks the rect inward by a given amount on all sides
    pub fn shrink(&self, amount: i32) -> Rect {
        Rect::new(
            Vec2::new(self.pos.x + amount, self.pos.y + amount),
            Vec2::new((self.size.x - amount * 2).max(0), (self.size.y - amount * 2).max(0)),
        )
    }

    /// Grows the rect outward by a given amount on all sides
    pub fn grow(&self, amount: i32) -> Rect {
        self.shrink(-amount)
    }
}