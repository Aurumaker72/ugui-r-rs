use crate::gfx::point::Point;

#[derive(Copy, Clone, Debug, Default)]
pub struct MouseState {
    pub pos: Point,
    pub lmb_down_pos: Point,
    pub lmb: bool,
    pub rmb: bool,
    pub mmb: bool,
}
