use crate::core::geo::{Alignment, Rect};

#[derive(Clone, PartialEq, Default)]
pub struct BaseControl {
    pub margin: Rect,
    pub horizontal_alignment: Alignment,
    pub vertical_alignment: Alignment,
    pub children: Vec<Box<BaseControl>>,
}

pub enum Control {
    None,
    Label { base: BaseControl, text: String },
}
