use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;

use crate::core::styles::{hex_color, Styles};
use crate::core::ugui::Ugui;
use crate::HWND;
use flagset::FlagSet;

use std::collections::HashMap;

#[derive(Copy, Clone, Default, Debug)]
struct ButtonState {
    /// The current visual state
    visual_state: VisualState,
}

pub const BUTTON_STATE_KEY: &str = "state";

pub fn button_style() -> FlagSet<Styles> {
    Styles::Visible | Styles::Enabled | Styles::Focusable
}
pub const BUTTON_CLICK: u64 = 50;

/// The message procedure implementation for a button
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
    let mut state: Option<ButtonState> = None;
    if let Some(data) = ugui.get_udata(hwnd, BUTTON_STATE_KEY) {
        state = Some(*(data.downcast::<ButtonState>().unwrap()));
    }

    match message {
        Message::StylesChanged => {
            let style = ugui.get_styles(hwnd);
            let mut state = ButtonState::default();

            if !style.contains(Styles::Enabled) {
                state.visual_state = VisualState::Disabled;
            } else {
                state.visual_state = VisualState::Normal;
            }

            ugui.set_udata(hwnd, BUTTON_STATE_KEY, Box::new(state));
        }
        Message::LmbDown => {
            state.unwrap().visual_state = VisualState::Active;
            ugui.set_udata(hwnd, BUTTON_STATE_KEY, Box::new(state));

            ugui.invalidate_rect(rect);
            ugui.capture_mouse(hwnd);
            ugui.send_message(ugui.root_hwnd(), Message::User(hwnd, BUTTON_CLICK));
        }
        Message::LmbUp => {
            if state.unwrap().visual_state == VisualState::Hover {
                state.unwrap().visual_state = VisualState::Normal;
            } else {
                state.unwrap().visual_state = VisualState::Hover;
            }
            ugui.set_udata(hwnd, BUTTON_STATE_KEY, Box::new(state));

            ugui.invalidate_rect(rect);
            ugui.uncapture_mouse(hwnd);
        }
        Message::MouseEnter => {
            if state.unwrap().visual_state == VisualState::Hover {
                state.unwrap().visual_state = VisualState::Active;
            } else {
                state.unwrap().visual_state = VisualState::Hover;
            }
            ugui.set_udata(hwnd, BUTTON_STATE_KEY, Box::new(state));

            ugui.invalidate_rect(rect);
        }
        Message::MouseLeave => {
            if state.unwrap().visual_state == VisualState::Active {
                state.unwrap().visual_state = VisualState::Hover;
            } else {
                state.unwrap().visual_state = VisualState::Normal;
            }
            ugui.set_udata(hwnd, BUTTON_STATE_KEY, Box::new(state));

            ugui.invalidate_rect(rect);
        }
        Message::Paint => {
            let rect = ugui.get_window_rect(hwnd);

            let colors = HashMap::from([
                (
                    VisualState::Normal,
                    (hex_color("#E1E1E1"), hex_color("#ADADAD")),
                ),
                (
                    VisualState::Hover,
                    (hex_color("#E5F1FB"), hex_color("#0078D7")),
                ),
                (
                    VisualState::Active,
                    (hex_color("#CCE4F7"), hex_color("#005499")),
                ),
                (
                    VisualState::Disabled,
                    (hex_color("#CCCCCC"), hex_color("#BFBFBF")),
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
