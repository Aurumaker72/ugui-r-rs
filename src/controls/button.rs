use crate::controls::visual_state::VisualState;
use crate::core::messages::Message;
use crate::window::HWND;
use crate::window::{base_proc, Ugui};
use num_traits::{FromPrimitive, ToPrimitive};
use sdl2::controller::Button;
use sdl2::pixels::Color;
use std::collections::HashMap;

fn hex(str: &str) -> Color {
    let r = &str[1..3];
    let g = &str[3..5];
    let b = &str[5..7];
    Color::RGB(
        u8::from_str_radix(r, 16).unwrap(),
        u8::from_str_radix(g, 16).unwrap(),
        u8::from_str_radix(b, 16).unwrap(),
    )
}

pub fn button_proc(ugui: &mut Ugui, hwnd: HWND, message: Message) -> u64 {
    match message {
        Message::LmbDown => {
            ugui.set_udata(hwnd, VisualState::Active.to_u64().unwrap());
            ugui.send_message(hwnd, Message::Paint);
            0
        }
        Message::LmbUp => {
            ugui.set_udata(hwnd, VisualState::Hover.to_u64().unwrap());
            ugui.send_message(hwnd, Message::Paint);
            0
        }
        Message::Paint => {
            let state: VisualState = FromPrimitive::from_u64(ugui.get_udata(hwnd)).unwrap();
            let rect = ugui.get_window_rect(hwnd);

            let colors = HashMap::from([
                (VisualState::Normal, (hex("#E1E1E1"), hex("#ADADAD"))),
                (VisualState::Hover, (hex("#E5F1FB"), hex("#0078D7"))),
                (VisualState::Active, (hex("#CCE4F7"), hex("#005499"))),
                (VisualState::Disabled, (hex("#CCCCCC"), hex("#BFBFBF"))),
            ]);

            ugui.paint_quad(rect, colors[&state].0, colors[&state].1, 1.0);
            0
        }
        _ => base_proc(ugui, hwnd, message),
    }
}
