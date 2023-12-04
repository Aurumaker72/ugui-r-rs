use crate::core::geo::{Alignment, Point, Rect};
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

impl Control {
    pub(crate) fn get_base(&mut self) -> &mut BaseControl {
        match self {
            Control::None => panic!("Invalid control"),
            Control::Label { base, .. } => base,
            Control::Stack { base, .. } => base,
        }
    }
    pub(crate) fn desired_size(&self) -> Point {}
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

    pub(crate) fn get_children(&mut self) -> Vec<Control> {
        let mut children = vec![];
        for child in &mut self.children {
            children.push(child.clone());
            let grandchildren = child.get_base().get_children();
            children.extend(grandchildren);
        }
        children.into()
    }

    pub(crate) fn do_layout(&mut self, parent_rect: Rect) {
        if !self.validated {
            println!("layout validated");
            self.validated = true
        }

        for child in &mut self.children {
            child.get_base().do_layout(self.computed_bounds);
        }
    }
}
