use crate::*;
use sdl2::pixels::Color;
/// The message procedure implementation for a top-level window
///
/// # Arguments
///
/// * `ugui`: A reference to the owner Ugui object
/// * `root_hwnd`: The root window's handle
/// * `hwnd`: The current window's handle
/// * `message`: The message
///
/// returns: u64 The message response
pub fn window_proc(ugui: &mut Ugui, root_hwnd: HWND, hwnd: HWND, message: Message) -> u64 {
    match message {
        Message::Create => {
            ugui.send_message(hwnd, Message::Paint);
        }
        Message::StylesChanged => {
            ugui.send_message(hwnd, Message::Paint);
        }
        Message::Paint => {
            let rect = ugui.get_window_rect(hwnd);

            ugui.paint_quad(
                rect,
                Color::RGB(240, 240, 240),
                Color::RGB(240, 240, 240),
                1.0,
            );
        }
        _ => {}
    }
    0
}
