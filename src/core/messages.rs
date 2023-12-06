use crate::core::geo::{Point, Rect};

#[derive(Clone, Debug)]
pub enum Message {
    MouseMove(Point),
}
