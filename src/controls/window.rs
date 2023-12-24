use crate::gfx::color::Color;
use crate::gfx::styles::Styles;
use crate::*;
use flagset::FlagSet;

pub fn window_style() -> FlagSet<Styles> {
    Styles::Visible | Styles::Enabled
}

/// The message procedure implementation for a top-level window
///
/// # Arguments
///
/// * `ugui`: A reference to the owner Ugui object
/// * `hwnd`: The source window's handle
/// * `message`: The message
///
/// returns: u64 The message response
pub fn window_proc(ugui: &mut Ugui, hwnd: HWND, message: Message) -> u64 {
    match message {
        Message::Paint => {
            let rect = ugui.get_window_rect(hwnd);

            ugui.paint_quad(
                rect,
                Color::rgb(240, 240, 240),
                Color::rgb(240, 240, 240),
                1.0,
            );
        }
        _ => {}
    }
    0
}
