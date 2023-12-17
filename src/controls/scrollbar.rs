use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;

use crate::core::styles::{hex_color, Styles};
use crate::core::ugui::Ugui;
use crate::HWND;
use flagset::FlagSet;
use num_traits::{FromPrimitive, ToPrimitive};

use sdl2::pixels::Color;
use std::collections::HashMap;
use std::ops::Deref;
use std::string::ToString;

#[derive(Copy, Clone, Default, Debug)]
struct ScrollbarState {
    /// The thumb's size in pixels
    size: i32,
    /// The scroll percentage
    value: f32,
    /// The current visual state
    visual_state: VisualState,
}

pub fn scrollbar_style() -> FlagSet<Styles> {
    Styles::Visible | Styles::Enabled | Styles::Focusable
}

pub const SCROLLBAR_STATE_KEY: &str = "state";
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
    let mut state: Option<ScrollbarState> = None;
    if let Some(data) = ugui.get_udata(hwnd, SCROLLBAR_STATE_KEY) {
        state = Some(*(data.downcast::<ScrollbarState>().unwrap()));
    }

    match message {
        Message::StylesChanged => {
            let style = ugui.get_styles(hwnd);
            let mut state = ScrollbarState::default();

            if !style.contains(Styles::Enabled) {
                state.visual_state = VisualState::Disabled;
            } else {
                state.visual_state = VisualState::Normal;
            }

            ugui.set_udata(hwnd, SCROLLBAR_STATE_KEY, Box::new(state));
        }
        Message::LmbDown => {
            ugui.invalidate_rect(rect);
            state.unwrap().visual_state = VisualState::Active;
            ugui.set_udata(hwnd, SCROLLBAR_STATE_KEY, Box::new(state));
            ugui.capture_mouse(hwnd);
        }
        Message::LmbUp => {
            ugui.invalidate_rect(rect);

            if state.unwrap().visual_state == VisualState::Hover {
                state.unwrap().visual_state = VisualState::Normal;
            } else {
                state.unwrap().visual_state = VisualState::Hover;
            }
            ugui.set_udata(hwnd, SCROLLBAR_STATE_KEY, Box::new(state));
            ugui.uncapture_mouse(hwnd);
        }
        Message::MouseEnter => {
            ugui.invalidate_rect(rect);

            if state.unwrap().visual_state == VisualState::Hover {
                state.unwrap().visual_state = VisualState::Active;
            } else {
                state.unwrap().visual_state = VisualState::Hover;
            }
            ugui.set_udata(hwnd, SCROLLBAR_STATE_KEY, Box::new(state));
        }
        Message::MouseLeave => {
            ugui.invalidate_rect(rect);

            if state.unwrap().visual_state == VisualState::Active {
                state.unwrap().visual_state = VisualState::Hover;
            } else {
                state.unwrap().visual_state = VisualState::Normal;
            }
            ugui.set_udata(hwnd, SCROLLBAR_STATE_KEY, Box::new(state));
        }
        Message::Paint => {
            let back_rect = ugui.get_window_rect(hwnd);
            let thumb_rect = ugui.get_window_rect(hwnd);

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
                back_rect,
                colors[state.unwrap().visual_state].0,
                Color::BLACK,
                0.0,
            );
            ugui.paint_quad(
                thumb_rect,
                colors[state.unwrap().visual_state].1,
                Color::BLACK,
                0.0,
            );
        }
        _ => {}
    }
    0
}
