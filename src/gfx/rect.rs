use sdl2::rect::Rect as SdlRect;
use crate::gfx::point::Point;

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn inflate(&self, v: f32) -> Rect {
        Rect {
            x: self.x - v,
            y: self.y - v,
            w: self.w + v * 2.0,
            h: self.h + v * 2.0,
        }
    }
    pub fn top_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
    pub fn top_right(&self) -> Point {
        Point {
            x: self.x + self.w,
            y: self.y,
        }
    }
    pub fn bottom_left(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + self.h,
        }
    }
    pub fn bottom_right(&self) -> Point {
        Point {
            x: self.x + self.w,
            y: self.y + self.h,
        }
    }
    pub fn size(&self) -> Point {
        Point {
            x: self.w,
            y: self.h,
        }
    }
    pub fn right(&self) -> f32 {
        self.bottom_right().x
    }

    pub fn bottom(&self) -> f32 {
        self.bottom_right().y
    }

    pub fn add(&self, other: Rect) -> Rect {
        Rect {
            x: self.x + other.x,
            y: self.y + other.y,
            ..*self
        }
    }

    pub fn add_pt(&self, other: Point) -> Rect {
        Rect {
            x: self.x + other.x,
            y: self.y + other.y,
            ..*self
        }
    }

    pub fn sub(&self, other: Rect) -> Rect {
        Rect {
            x: self.x - other.x,
            y: self.y - other.y,
            ..*self
        }
    }

    pub fn sub_pt(&self, other: Point) -> Rect {
        Rect {
            x: self.x - other.x,
            y: self.y - other.y,
            ..*self
        }
    }

    pub fn intersects(&self, other: Rect) -> bool {
        !(self.x > other.right()
            || self.right() < other.x
            || self.y > other.bottom()
            || self.bottom() < other.y)
    }
    pub fn to_sdl(&self) -> SdlRect {
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}
