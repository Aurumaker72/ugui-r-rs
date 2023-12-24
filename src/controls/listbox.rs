use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;

use crate::core::ugui::Ugui;
use crate::HWND;
use flagset::FlagSet;

use crate::gfx::styles::{ Styles};
use std::collections::HashMap;
use crate::gfx::color::Color;

#[derive(Clone, Default, Debug)]
struct ListboxState {
    /// The current visual state
    visual_state: VisualState,
    /// The items
    items: Vec<String>,
}

pub fn listbox_style() -> FlagSet<Styles> {
    Styles::Visible | Styles::Enabled | Styles::Focusable
}
pub const LISTBOX_SELECTION_CHANGED: u64 = 50;

/// The message procedure implementation for a listbox
///
/// # Arguments
///
/// * `ugui`: A reference to the owner Ugui object
/// * `hwnd`: The source window's handle
/// * `message`: The message
///
/// returns: u64 The message response
pub fn button_proc(ugui: &mut Ugui, hwnd: HWND, message: Message) -> u64 {
    let rect = ugui.get_window_rect(hwnd);
    let mut state: Option<ListboxState> = None;
    if let Some(data) = ugui.get_udata(hwnd) {
        state = Some(*(data.downcast::<ListboxState>().unwrap()));
    }

    match message {
        Message::StylesChanged => {
            let style = ugui.get_styles(hwnd);
            let mut state = ListboxState::default();

            if !style.contains(Styles::Enabled) {
                state.visual_state = VisualState::Disabled;
            } else {
                state.visual_state = VisualState::Normal;
            }

            ugui.set_udata(hwnd, Some(Box::new(state)));
        }
        Message::LmbDown => {
            state.as_mut().unwrap().visual_state = VisualState::Active;
            ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
            ugui.invalidate_rect(rect);
            ugui.capture_mouse(hwnd);
        }
        Message::LmbUp => {
            if state.as_mut().unwrap().visual_state == VisualState::Hover {
                state.as_mut().unwrap().visual_state = VisualState::Normal;
            } else {
                state.as_mut().unwrap().visual_state = VisualState::Hover;
            }
            ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
            ugui.invalidate_rect(rect);
            ugui.uncapture_mouse(hwnd);
        }
        Message::MouseEnter => {
            if state.as_mut().unwrap().visual_state == VisualState::Hover {
                state.as_mut().unwrap().visual_state = VisualState::Active;
            } else {
                state.as_mut().unwrap().visual_state = VisualState::Hover;
            }
            ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
            ugui.invalidate_rect(rect);
        }
        Message::MouseLeave => {
            if state.as_mut().unwrap().visual_state == VisualState::Active {
                state.as_mut().unwrap().visual_state = VisualState::Hover;
            } else {
                state.as_mut().unwrap().visual_state = VisualState::Normal;
            }
            ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
            ugui.invalidate_rect(rect);
        }
        Message::Paint => {
            let rect = ugui.get_window_rect(hwnd);

            let colors = HashMap::from([
                (
                    VisualState::Normal,
                    (Color::hex("#E1E1E1"), Color::hex("#ADADAD")),
                ),
                (
                    VisualState::Hover,
                    (Color::hex("#E5F1FB"), Color::hex("#0078D7")),
                ),
                (
                    VisualState::Active,
                    (Color::hex("#CCE4F7"), Color::hex("#005499")),
                ),
                (
                    VisualState::Disabled,
                    (Color::hex("#CCCCCC"), Color::hex("#BFBFBF")),
                ),
            ]);
            ugui.paint_quad(
                rect,
                colors[&state.as_ref().unwrap().visual_state].0,
                colors[&state.as_ref().unwrap().visual_state].1,
                1.0,
            );
        }
        _ => {}
    }
    0
}
