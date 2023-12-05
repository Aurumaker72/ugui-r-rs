use crate::core::geo::{Alignment, Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use sdl2::ttf::Font;

#[derive(Clone, PartialEq, Debug)]
pub struct BaseControl {
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
    pub(crate) fn get_base_mut(&mut self) -> &mut BaseControl {
        match self {
            Control::Label { base, .. } => base,
            Control::Stack { base, .. } => base,
            _ => panic!("Expected control, got none"),
        }
    }
    pub(crate) fn get_base(&self) -> &BaseControl {
        match self {
            Control::Label { base, .. } => base,
            Control::Stack { base, .. } => base,
            _ => panic!("Expected control, got none"),
        }
    }
    pub(crate) fn compute_desired_size<'a>(&self, font: &Font<'a, 'static>) -> Point {
        match self {
            Control::Label { base: _, text } => {
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
    fn get_base_layout_bounds<'a>(&self, parent_rect: Rect, font: &Font<'a, 'static>) -> Rect {
        let base = self.get_base();
        let size = self.compute_desired_size(font);

        let mut base_rect = Rect {
            x: parent_rect.x,
            y: parent_rect.y,
            w: size.x,
            h: size.y,
        };
        if base.h_align == Alignment::Center {
            base_rect.x = parent_rect.x + parent_rect.w / 2.0 - size.x / 2.0;
        }
        if base.h_align == Alignment::End {
            base_rect.x = parent_rect.x + parent_rect.w - size.x;
        }
        if base.h_align == Alignment::Fill {
            base_rect.w = parent_rect.w;
        }
        if base.v_align == Alignment::Center {
            base_rect.y = parent_rect.y + parent_rect.h / 2.0 - size.y / 2.0;
        }
        if base.v_align == Alignment::End {
            base_rect.y = parent_rect.y + parent_rect.h - size.y;
        }
        if base.v_align == Alignment::Fill {
            base_rect.h = parent_rect.h;
        }
        base_rect
    }

    pub(crate) fn compute_layout_bounds<'a>(
        &self,
        parent_rect: Rect,
        font: &Font<'a, 'static>,
    ) -> (Rect, Vec<Rect>) {
        let base = self.get_base();
        let layout_bounds = self.get_base_layout_bounds(parent_rect, font);

        match self {
            Control::Stack { base, .. } => {
                // Stack child layout computation:
                // take base layout bounds and offset them by accumulated height offset

                let mut h = 0.0;
                for i in 0..base.children.len() {
                    let original_rect = base.children[i].get_base().computed_bounds;
                    let rect = &mut (original_rect.clone());
                    let child_base = base.children[i].get_base();

                    let available_rect = Rect {
                        x: base.computed_bounds.x,
                        y: base.computed_bounds.y + h,
                        w: base.computed_bounds.w,
                        h: h,
                    };

                    // We need to position the rect inside our available item space according to the child's alignment properties
                    // if child_base.h_align == Alignment::Center {
                    *rect = available_rect;
                    // }

                    // rect.y += h;
                    h += original_rect.h;
                }

                (layout_bounds, vec![])
            }
            _ => (layout_bounds, vec![]),
        }
    }
    pub(crate) fn render(&self, window_canvas: &mut WindowCanvas) {
        let base = self.get_base();

        let color = match self {
            Control::Stack { .. } => Color::RED,
            Control::Label { .. } => Color::WHITE,
            _ => Color::MAGENTA,
        };

        window_canvas.set_draw_color(color);
        window_canvas
            .draw_rect(self.get_base().computed_bounds.to_sdl())
            .unwrap();

        for child in &base.children {
            child.render(window_canvas);
        }
    }
    pub(crate) fn do_layout<'a>(&mut self, parent_rect: Rect, font: &Font<'a, 'static>) {
        let cloned = self.clone();
        let base = self.get_base_mut();

        if base.validated {
            return;
        }

        // Compute the base layout bounds, and apply them
        base.computed_bounds = cloned.get_base_layout_bounds(parent_rect, font);

        for child in &mut base.children {
            child.do_layout(base.computed_bounds, font);
        }

        // Control-specific logic: we reposition childrens' bounds after their layout is finished
        // (this is only reached after all children are laid out)
        match cloned {
            Control::Stack { .. } => {
                // Accumulate height (needed for vertical stack)
                let mut current_height = 0.0;
                for child in &mut base.children {
                    // Recompute layout bounds inside limited region
                    let clone = child.clone();
                    let child_base = child.get_base_mut();
                    child_base.computed_bounds = clone.get_base_layout_bounds(
                        Rect {
                            x: base.computed_bounds.x,
                            y: base.computed_bounds.y + current_height,
                            w: base.computed_bounds.w,
                            h: clone.get_base().computed_bounds.h,
                        },
                        font,
                    );
                    current_height += clone.get_base().computed_bounds.h;
                }
            }
            _ => {}
        }

        base.validated = true;
    }
}

impl BaseControl {
    pub fn default() -> Self {
        BaseControl {
            h_align: Default::default(),
            v_align: Default::default(),
            children: Default::default(),
            visible: true,
            validated: false,
            computed_bounds: Default::default(),
        }
    }
    pub fn new(h_align: Alignment, v_align: Alignment, children: Vec<Control>) -> Self {
        BaseControl {
            h_align,
            v_align,
            children,
            visible: true,
            validated: false,
            computed_bounds: Default::default(),
        }
    }

    pub(crate) fn _get_children(&self) -> Vec<Control> {
        let mut children = vec![];
        for child in &self.children {
            children.push(child.clone());
            let grandchildren = child.get_base()._get_children();
            children.extend(grandchildren);
        }
        children.into()
    }
}
