use sdl2::rect::Point as SdlPoint;
use sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn to_sdl(&self) -> SdlPoint {
        SdlPoint::new(self.x as i32, self.y as i32)
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

#[derive(Clone, Copy, Default)]
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

    pub fn to_sdl(&self) -> SdlRect {
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}

pub fn remap(value: f32, from1: f32, to1: f32, from2: f32, to2: f32) -> f32 {
    (value - from1) / (to1 - from1) * (to2 - from2) + from2
}
