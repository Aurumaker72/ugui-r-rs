use crate::core::geo::{Alignment, Point, Rect};
use sdl2::sys::ttf::TTF_Font;
use sdl2::ttf::Font;
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
            Control::Label { base, .. } => base,
            Control::Stack { base, .. } => base,
            _ => panic!("Expected control, got none"),
        }
    }
    pub(crate) fn compute_desired_size<'a>(&self, font: &Font<'a, 'static>) -> Point {
        match self {
            Control::Label { base, text } => {
                // Label measurement: string size with current font
                let size = font.size_of(text).unwrap();
                Point {
                    x: size.0 as f32,
                    y: size.1 as f32,
                }
            }
            Control::Stack { base, .. } => {
                // Stack measurement: sum of h component of all children, max of w component
                let children_sizes = base.children.iter().map(|x| x.compute_desired_size(font));

                Point {
                    x: children_sizes
                        .clone()
                        .max_by(|a, b| a.x.total_cmp(&b.x))
                        .unwrap()
                        .x,
                    y: children_sizes.clone().map(|x| x.y).sum(),
                }
            }
            _ => panic!("Not implemented for {:?}", self),
        }
    }
    pub(crate) fn do_layout<'a>(&mut self, parent_rect: Rect, font: &Font<'a, 'static>) {
        let cloned = self.clone();
        let mut base = self.get_base();

        if !base.validated {
            let size = cloned.compute_desired_size(font);

            base.computed_bounds = Rect {
                x: parent_rect.x,
                y: parent_rect.y,
                w: size.x,
                h: size.y,
            };
            println!("validated, size: {:?}", size);
            base.validated = true
        }

        for child in &mut base.children {
            child.do_layout(base.computed_bounds, font);
        }
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

    pub(crate) fn get_children(&mut self) -> Vec<Control> {
        let mut children = vec![];
        for child in &mut self.children {
            children.push(child.clone());
            let grandchildren = child.get_base().get_children();
            children.extend(grandchildren);
        }
        children.into()
    }
}
