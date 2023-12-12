use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;
use crate::core::messages::Message::StylesChanged;
use crate::core::styles::{hex_color, Styles};
use crate::core::ugui::Ugui;
use crate::HWND;
use num_traits::{FromPrimitive, ToPrimitive};
use sdl2::controller::Button;
use sdl2::pixels::Color;
use std::collections::HashMap;

pub const TEXTBOX_CHANGED: u64 = 51;

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
    match message {
        Message::StylesChanged => {
            let style = ugui.get_styles(hwnd);

            if !style.contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
            }
        }
        Message::Focus => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
                return 0;
            }
            ugui.set_udata(hwnd, VisualState::Active.to_u64().unwrap());
        }
        Message::Unfocus => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
                return 0;
            }
            ugui.set_udata(hwnd, VisualState::Normal.to_u64().unwrap());
        }
        Message::MouseMove => {
            // TODO: Caret control
        }
        Message::MouseEnter => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
                return 0;
            }

            let state: VisualState = FromPrimitive::from_u64(ugui.get_udata(hwnd)).unwrap();
            if state == VisualState::Normal {
                ugui.set_udata(hwnd, VisualState::Hover.to_u64().unwrap());
            }
            ugui.send_message(hwnd, Message::Paint);
        }
        Message::MouseLeave => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(hwnd, VisualState::Disabled.to_u64().unwrap());
                return 0;
            }
            let state: VisualState = FromPrimitive::from_u64(ugui.get_udata(hwnd)).unwrap();

            if state == VisualState::Hover {
                ugui.set_udata(hwnd, VisualState::Normal.to_u64().unwrap());
            }
            ugui.send_message(hwnd, Message::Paint);
        }
        Message::Paint => {
            let state: VisualState = FromPrimitive::from_u64(ugui.get_udata(hwnd)).unwrap();
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

            ugui.paint_quad(rect, colors[&state].0, colors[&state].1, 1.0);
        }
        _ => {}
    }
    0
}
