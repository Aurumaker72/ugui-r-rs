use crate::gfx::color::Color;
use crate::gfx::rect::Rect;

#[derive(Copy, Clone, Debug)]
pub struct QuadDrawOp {
    pub rect: Rect,
    pub back_color: Color,
    pub border_color: Color,
    pub border_size: f32,
}
