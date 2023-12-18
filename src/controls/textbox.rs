use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;

use crate::core::styles::{hex_color, Styles};
use crate::core::ugui::Ugui;
use crate::HWND;
use flagset::FlagSet;
use num_traits::{FromPrimitive, ToPrimitive};

use std::collections::HashMap;
use std::ops::Deref;

#[derive(Copy, Clone, Default, Debug)]
struct TextboxState {
    /// The current visual state
    visual_state: VisualState,
}

pub const TEXTBOX_CHANGED: u64 = 51;
pub fn textbox_style() -> FlagSet<Styles> {
    Styles::Visible | Styles::Enabled | Styles::Focusable
}

/// The message procedure implementation for a textbox
///
/// # Arguments
///
/// * `ugui`: A reference to the owner Ugui object
/// * `hwnd`: The source window's handle
/// * `message`: The message
///
/// returns: u64 The message response
pub fn textbox_proc(ugui: &mut Ugui, hwnd: HWND, message: Message) -> u64 {
    let rect = ugui.get_window_rect(hwnd);
    let mut state: Option<TextboxState> = None;
    if let Some(data) = ugui.get_udata(hwnd) {
        state = Some(*(data.downcast::<TextboxState>().unwrap()));
    }

    match message {
        Message::StylesChanged => {
            let style = ugui.get_styles(hwnd);
            let mut state = TextboxState::default();

            if !style.contains(Styles::Enabled) {
                state.visual_state = VisualState::Disabled;
            } else {
                state.visual_state = VisualState::Normal;
            }

            ugui.set_udata(hwnd, Some(Box::new(state)));
        }
        Message::Focus => {
            state.as_mut().unwrap().visual_state = VisualState::Active;
            ugui.invalidate_rect(rect);
            ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
        }
        Message::Unfocus => {
            state.as_mut().unwrap().visual_state = VisualState::Normal;
            ugui.invalidate_rect(rect);
            ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
        }
        Message::MouseMove => {
            // TODO: Caret control
        }
        Message::MouseEnter => {
            if state.as_mut().unwrap().visual_state == VisualState::Normal {
                state.as_mut().unwrap().visual_state = VisualState::Hover;
                ugui.invalidate_rect(rect);
                ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
            }
        }
        Message::MouseLeave => {
            if state.as_mut().unwrap().visual_state == VisualState::Hover {
                state.as_mut().unwrap().visual_state = VisualState::Normal;
                ugui.invalidate_rect(rect);
                ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
            }
        }
        Message::LmbDown => {
            ugui.capture_mouse(hwnd);
        }
        Message::LmbUp => {
            ugui.uncapture_mouse(hwnd);
        }
        Message::TextInput => {
            println!("{}", ugui.typed_text());
        }
        Message::Paint => {
            let rect = ugui.get_window_rect(hwnd);

            let colors = HashMap::from([
                (
                    VisualState::Normal,
                    (hex_color("#FFFFFF"), hex_color("#7A7A7A")),
                ),
                (
                    VisualState::Hover,
                    (hex_color("#FFFFFF"), hex_color("#171717")),
                ),
                (
                    VisualState::Active,
                    (hex_color("#FFFFFF"), hex_color("#0078D7")),
                ),
                (
                    VisualState::Disabled,
                    (hex_color("#FFFFFF"), hex_color("#CCCCCC")),
                ),
            ]);

            ugui.paint_quad(
                rect,
                colors[&state.unwrap().visual_state].0,
                colors[&state.unwrap().visual_state].1,
                1.0,
            );
        }
        _ => {}
    }
    0
}
