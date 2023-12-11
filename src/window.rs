extern crate sdl2;
use crate::core::ugui::Ugui;
use crate::WNDPROC;
use crate::core::geo::Alignment;
use crate::core::geo::{Point, Rect};
use crate::core::messages::Message;
use crate::core::styles::Styles;
use flagset::FlagSet;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::collections::HashMap;
use std::path::Path;
use crate::HWND;

#[derive(Clone)]
pub(crate) struct Window {
    pub hwnd: HWND,
    pub class: String,
    pub caption: String,
    pub styles: FlagSet<Styles>,
    pub rect: Rect,
    pub parent: Option<HWND>,
    pub procedure: WNDPROC,
    pub state_0: u64,
}

pub fn base_proc(ugui: &mut Ugui, hwnd: HWND, message: Message) -> u64 {
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
