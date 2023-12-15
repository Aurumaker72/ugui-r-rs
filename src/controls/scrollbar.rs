use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;

use crate::core::styles::{hex_color, Styles};
use crate::core::ugui::Ugui;
use crate::HWND;
use flagset::FlagSet;
use num_traits::{FromPrimitive, ToPrimitive};

use std::collections::HashMap;
use std::ops::Deref;
use std::string::ToString;
use sdl2::pixels::Color;

pub fn scrollbar_style() -> FlagSet<Styles> {
    Styles::Visible | Styles::Enabled | Styles::Focusable
}

pub const SCROLLBAR_VALUE_KEY: &str = "value";
pub const SCROLLBAR_CHANGED: u64 = 53;

/// The message procedure implementation for a scrollbar
///
/// # Arguments
///
/// * `ugui`: A reference to the owner Ugui object
/// * `hwnd`: The source window's handle
/// * `message`: The message
///
/// returns: u64 The message response
pub fn scrollbar_proc(ugui: &mut Ugui, hwnd: HWND, message: Message) -> u64 {
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
        Message::LmbDown => {
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
            ugui.capture_mouse(hwnd);
        }
        Message::LmbUp => {
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
            } else {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Hover),
                );
            }
            ugui.uncapture_mouse(hwnd);
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

            if visual_state.is_some_and(|x| x == VisualState::Hover) {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Active),
                );
            } else {
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

            if visual_state.is_some_and(|x| x == VisualState::Active) {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Hover),
                );
            } else {
                ugui.set_udata(
                    hwnd,
                    "visual_state".to_string(),
                    Box::new(VisualState::Normal),
                );
            }
        }
        Message::Paint => {
            let rect = ugui.get_window_rect(hwnd);

            let colors = HashMap::from([
                (
                    VisualState::Normal,
                    (hex_color("#F0F0F0"), hex_color("#CDCDCD")),
                ),
                (
                    VisualState::Hover,
                    (hex_color("#F0F0F0"), hex_color("#A6A6A6")),
                ),
                (
                    VisualState::Active,
                    (hex_color("#F0F0F0"), hex_color("#606060")),
                ),
                (
                    VisualState::Disabled,
                    (hex_color("#F0F0F0"), hex_color("#C0C0C0")),
                ),
            ]);

            ugui.paint_quad(
                rect,
                colors[visual_state.as_ref().unwrap()].0,
                Color::BLACK,
                0.0,
            );
        }
        _ => {}
    }
    0
}
