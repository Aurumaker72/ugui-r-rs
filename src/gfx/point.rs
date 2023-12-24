use crate::gfx::rect::Rect;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new_i(x: i32, y: i32) -> Point {
        Point {
            x: x as f32,
            y: y as f32,
        }
    }
    pub fn inside(&self, rect: Rect) -> bool {
        self.x > rect.x && self.x < rect.right() && self.y > rect.y && self.y < rect.bottom()
    }
    pub fn add(&self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn sub(&self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    pub fn dist(&self, other: Point) -> f32 {
        ((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)).sqrt()
    }
}
