use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;

use crate::core::styles::{hex_color, Styles};
use crate::core::ugui::Ugui;
use crate::HWND;
use flagset::FlagSet;
use num_traits::{FromPrimitive, ToPrimitive};

use std::collections::HashMap;

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

    match message {
        Message::StylesChanged => {
            let style = ugui.get_styles(hwnd);

            if !style.contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
            }
        }
        Message::LmbDown => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
                return 0;
            }
            ugui.invalidate_rect(rect);
            ugui.set_udata(hwnd, VisualState::Active.to_u64().unwrap());
            ugui.capture_mouse(hwnd);
            ugui.send_message(ugui.root_hwnd(), Message::User(hwnd, BUTTON_CLICK));
        }
        Message::LmbUp => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
                return 0;
            }
            ugui.invalidate_rect(rect);

            let state: VisualState = FromPrimitive::from_u64(ugui.get_udata(hwnd)).unwrap();

            if state == VisualState::Hover {
                ugui.set_udata(hwnd, VisualState::Normal.to_u64().unwrap());
            } else {
                ugui.set_udata(hwnd, VisualState::Hover.to_u64().unwrap());
            }
            ugui.uncapture_mouse(hwnd);
        }
        Message::MouseEnter => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
                return 0;
            }
            ugui.invalidate_rect(rect);
            let state: VisualState = FromPrimitive::from_u64(ugui.get_udata(hwnd)).unwrap();

            if state == VisualState::Hover {
                ugui.set_udata(hwnd, VisualState::Active.to_u64().unwrap());
            } else {
                ugui.set_udata(hwnd, VisualState::Hover.to_u64().unwrap());
            }
        }
        Message::MouseLeave => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
                return 0;
            }
            ugui.invalidate_rect(rect);
            let state: VisualState = FromPrimitive::from_u64(ugui.get_udata(hwnd)).unwrap();

            if state == VisualState::Active {
                ugui.set_udata(hwnd, VisualState::Hover.to_u64().unwrap());
            } else {
                ugui.set_udata(hwnd, VisualState::Normal.to_u64().unwrap());
            }
        }
        Message::Paint => {
            let state: VisualState = FromPrimitive::from_u64(ugui.get_udata(hwnd)).unwrap();
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

            ugui.paint_quad(rect, colors[&state].0, colors[&state].1, 1.0);
        }
        _ => {}
    }
    0
}
