use crate::core::geo::{Alignment, Rect};
use std::cell::Ref;
use std::rc::Rc;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct BaseControl {
    pub margin: Rect,
    pub horizontal_alignment: Alignment,
    pub vertical_alignment: Alignment,
    pub children: Vec<Rc<Control>>,
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
