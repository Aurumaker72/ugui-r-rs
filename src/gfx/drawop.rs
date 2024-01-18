use crate::gfx::alignment::Alignment;
use crate::gfx::color::Color;
use crate::gfx::rect::Rect;

#[derive(Copy, Clone, Debug)]
pub struct QuadDrawOp {
    pub rect: Rect,
    pub clip_rect: Option<Rect>,
    pub back_color: Color,
    pub border_color: Color,
    pub border_size: f32,
}

#[derive(Clone, Debug)]
pub struct TextDrawOp {
    pub rect: Rect,
    pub clip_rect: Option<Rect>,
    pub color: Color,
    pub text: String,
    pub h_align: Alignment,
    pub v_align: Alignment,
    pub size: f32,
    pub line_height: f32,
}