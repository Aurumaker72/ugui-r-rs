use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;

use crate::core::styles::{hex_color, Styles};
use crate::core::ugui::Ugui;
use crate::HWND;
use flagset::FlagSet;

use crate::core::geo::Rect;
use sdl2::pixels::Color;
use std::collections::HashMap;

#[derive(Copy, Clone, Default, Debug)]
struct ScrollbarState {
    /// The thumb's size in relation to the scrollbar's height
    size: f32,
    /// The scroll percentage
    value: f32,
    /// The current visual state
    visual_state: VisualState,
}

pub fn scrollbar_style() -> FlagSet<Styles> {
    Styles::Visible | Styles::Enabled | Styles::Focusable
}
pub fn scrollbar_set(ugui: &mut Ugui, hwnd: HWND, size: f32, value: f32) {
    if let Some(data) = ugui.get_udata(hwnd) {
        let state = *(data.downcast::<ScrollbarState>().unwrap());
        ugui.set_udata(
            hwnd,
            Some(Box::new(ScrollbarState {
                size,
                value,
                ..state
            })),
        );
        ugui.invalidate_hwnd(hwnd);
    }
}
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
    if let Some(data) = ugui.get_udata(hwnd) {
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

            ugui.set_udata(hwnd, Some(Box::new(state)));
        }
        Message::LmbDown => {
            state.as_mut().unwrap().visual_state = VisualState::Active;
            ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
            ugui.capture_mouse(hwnd);
            ugui.invalidate_rect(rect);
        }
        Message::LmbUp => {
            if state.as_mut().unwrap().visual_state == VisualState::Hover {
                state.as_mut().unwrap().visual_state = VisualState::Normal;
            } else {
                state.as_mut().unwrap().visual_state = VisualState::Hover;
            }
            ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
            ugui.uncapture_mouse(hwnd);
            ugui.invalidate_rect(rect);
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
        Message::MouseMove => {
            if ugui.get_capture().is_some_and(|x| x == hwnd) {
                let rect = ugui.get_window_rect(hwnd);
                let pos = ugui.mouse_state().pos.sub(rect.top_left());

                let frac = pos.y / rect.h;
                state.as_mut().unwrap().value = frac.clamp(0.0, 1.0);
                ugui.set_udata(hwnd, Some(Box::new(state.unwrap())));
                ugui.invalidate_hwnd(hwnd);
            }
        }
        Message::Paint => {
            let back_rect = ugui.get_window_rect(hwnd);
            let thumb_rect = Rect {
                x: back_rect.x,
                y: back_rect.y + (back_rect.h * state.unwrap().value),
                w: back_rect.w,
                h: back_rect.h * state.unwrap().size,
            };

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

            let visual_state = if ugui.get_capture().is_some_and(|x| x == hwnd) {
                VisualState::Active
            } else {
                state.unwrap().visual_state
            };

            ugui.paint_quad(
                back_rect,
                colors[&visual_state].0,
                Color::RED,
                0.0,
            );
            ugui.paint_quad(
                thumb_rect,
                colors[&visual_state].1,
                Color::RED,
                0.0,
            );
        }
        _ => {}
    }
    0
}
