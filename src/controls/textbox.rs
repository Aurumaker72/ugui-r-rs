use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;

use crate::core::styles::{hex_color, Styles};
use crate::core::ugui::Ugui;
use crate::HWND;
use flagset::FlagSet;
use num_traits::{FromPrimitive, ToPrimitive};

use std::collections::HashMap;
use std::ops::Deref;

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
    let mut visual_state: Option<VisualState> = None;
    if let Some(data) = ugui.get_udata(hwnd, "visual_state") {
        visual_state = Some(*(data.downcast::<VisualState>().unwrap()));
    }

    match message {
        Message::StylesChanged => {
            let style = ugui.get_styles(hwnd);

            if !style.contains(Styles::Enabled) {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Disabled),
                );
            } else {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Normal),
                );
            }
        }
        Message::Focus => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Disabled),
                );
                return 0;
            }
            ugui.invalidate_rect(rect);
            ugui.set_udata(
                hwnd,
                "visual_state".to_string(),
                Box::new(VisualState::Active),
            );
        }
        Message::Unfocus => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Disabled),
                );
                return 0;
            }
            ugui.invalidate_rect(rect);
            ugui.set_udata(
                hwnd,
                "visual_state".to_string(),
                Box::new(VisualState::Normal),
            );
        }
        Message::MouseMove => {
            // TODO: Caret control
        }
        Message::MouseEnter => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Disabled),
                );
                return 0;
            }
            ugui.invalidate_rect(rect);
            if visual_state.is_some_and(|x| x == VisualState::Normal) {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Hover),
                );
            }
        }
        Message::MouseLeave => {
            if !ugui.get_styles(hwnd).contains(Styles::Enabled) {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Disabled),
                );
                return 0;
            }
            ugui.invalidate_rect(rect);

            if visual_state.is_some_and(|x| x == VisualState::Hover) {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Normal),
                );
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
                colors[visual_state.as_ref().unwrap()].0,
                colors[visual_state.as_ref().unwrap()].1,
                1.0,
            );
        }
        _ => {}
    }
    0
}
