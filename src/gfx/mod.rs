pub mod point;
pub mod rect;
pub mod styles;
pub mod alignment;
pub mod color;
pub mod keycode;

pub fn remap(value: f32, from1: f32, to1: f32, from2: f32, to2: f32) -> f32 {
    (value - from1) / (to1 - from1) * (to2 - from2) + from2
}
