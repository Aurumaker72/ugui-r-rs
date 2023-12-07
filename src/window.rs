extern crate sdl2;
use crate::core::geo::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::path::Path;
type HWND = usize;

pub const CENTER_SCREEN: f32 = -1.0;

struct Window {
    hwnd: HWND,
    class: String,
    caption: String,
    style: u32,
    rect: Rect,
    parent: Option<HWND>,
}

/// The global application context, roughly equivalent to a WinAPI INSTANCE
#[derive(Default)]
pub struct Ugui {
    windows: Vec<Window>,
}

impl Ugui {
    pub fn create_window(
        &mut self,
        class: String,
        caption: String,
        style: u32,
        rect: Rect,
        parent: Option<HWND>,
    ) -> Option<HWND> {
        self.windows.push(Window {
            hwnd: self.windows.len() + 1,
            class,
            caption,
            style,
            rect,
            parent,
        });
        Some(self.windows.last().unwrap().hwnd)
    }

    pub fn show_window(&self, hwnd: HWND) {
        let window = self.windows.iter().find(|x| x.hwnd == hwnd).unwrap();

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        let mut window_builder = &mut video_subsystem.window(
            &window.caption,
            window.rect.w as u32,
            window.rect.h as u32,
        );

        window_builder
            .position(window.rect.x as i32, window.rect.y as i32)
            .opengl()
            .resizable();

        if window.rect.x == CENTER_SCREEN && window.rect.y == CENTER_SCREEN {
            window_builder = window_builder.position_centered();
        }

        let mut window = window_builder.build().map_err(|e| e.to_string()).unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string()).unwrap();

        // TODO: fix this magic path bullshit
        let font = ttf_context
            .load_font(Path::new("../../src/skin/segoe.ttf"), 16)
            .unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            canvas.present();
        }
    }
}
