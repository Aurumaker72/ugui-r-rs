use crate::core::messages::Message;
use crate::window::HWND;
use crate::window::{default_proc, Ugui};
use sdl2::pixels::Color;

pub fn button_proc(ugui: &mut Ugui, hwnd: HWND, message: Message) -> u64 {
    match message {
        Message::Paint => {
            let rect = ugui.get_window_rect(hwnd);
            ugui.paint_quad(rect, Color::RGB(255, 0, 0), Color::RGB(255, 55, 55), 1.0);
        }
        _ => return default_proc(ugui, hwnd, message),
    }
    0
}
