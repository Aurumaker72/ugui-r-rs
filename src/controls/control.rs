use crate::core::geo::{Alignment, Rect};
use std::cell::Ref;
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
pub struct BaseControl {
    margin: Rect,
    h_align: Alignment,
    v_align: Alignment,
    visible: bool,
    children: Vec<Control>,
    pub(crate) validated: bool,
    // The absolute bounds, as computed by the layout engine
    pub(crate) computed_bounds: Rect,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Control {
    None,
    Label { base: BaseControl, text: String },
    Stack { base: BaseControl },
}

pub fn get_base(control: Control) -> Option<BaseControl> {
    match control {
        Control::None => None,
        Control::Label { base, .. } => Some(base),
        Control::Stack { base, .. } => Some(base),
    }
}

impl BaseControl {
    pub fn default() -> Self {
        BaseControl {
            margin: Default::default(),
            h_align: Default::default(),
            v_align: Default::default(),
            children: Default::default(),
            visible: true,
            validated: false,
            computed_bounds: Default::default(),
        }
    }
    pub fn new(
        margin: Rect,
        h_align: Alignment,
        v_align: Alignment,
        children: Vec<Control>,
    ) -> Self {
        BaseControl {
            margin,
            h_align,
            v_align,
            children,
            visible: true,
            validated: false,
            computed_bounds: Default::default(),
        }
    }
    pub(crate) fn get_children(&self) -> Vec<Control> {
        let mut children = vec![];
        for child in &self.children {
            children.push(child.clone());
            let grandchildren = get_base(child.clone()).unwrap().get_children();
            children.extend(grandchildren);
        }
        children.into()
    }
}
