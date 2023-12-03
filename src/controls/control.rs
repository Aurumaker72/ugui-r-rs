use crate::core::geo::{Alignment, Rect};
use std::cell::Ref;
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
pub struct BaseControl {
    margin: Rect,
    h_align: Alignment,
    v_align: Alignment,
    visible: bool,
    children: Vec<Rc<Control>>,
    validated: bool,
}

#[derive(PartialEq, Debug)]
pub enum Control {
    None,
    Label { base: BaseControl, text: String },
    Stack { base: BaseControl },
}
pub fn get_base(control: &Control) -> Option<&BaseControl> {
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
        }
    }
    pub fn new(
        margin: Rect,
        h_align: Alignment,
        v_align: Alignment,
        children: Vec<Rc<Control>>,
    ) -> Self {
        BaseControl {
            margin,
            h_align,
            v_align,
            children,
            visible: true,
            validated: false,
        }
    }
    pub(crate) fn get_children(&self) -> Vec<Rc<Control>> {
        let mut children = vec![];
        for child in &self.children {
            children.push(child.clone());
            let grandchildren = get_base(child.as_ref()).unwrap().get_children();
            children.extend(grandchildren);
        }
        children
    }
}
