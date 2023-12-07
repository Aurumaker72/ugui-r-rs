extern crate sdl2;
use crate::core::geo::Rect;
use crate::core::messages::Message;
use crate::core::styles::Styles;
use flagset::FlagSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use std::path::Path;

pub type HWND = usize;
pub const CENTER_SCREEN: f32 = -1.0;

struct Window {
    hwnd: HWND,
    class: String,
    caption: String,
    styles: FlagSet<Styles>,
    rect: Rect,
    parent: Option<HWND>,
    procedure: fn(HWND, Message) -> u64,
}

impl Window {
    fn top_level(&self) -> bool {
        self.parent.is_none()
    }
}

fn default_proc(hwnd: HWND, message: Message) -> u64 {
    return 0;
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
        styles: FlagSet<Styles>,
        rect: Rect,
        parent: Option<HWND>,
        procedure: Option<fn(HWND, Message) -> u64>,
    ) -> Option<HWND> {
        let hwnd = self.windows.len();
        let mut actual_procedure: fn(HWND, Message) -> u64;

        // All windows need a procedure, so we use the default one if user can't bother to supply one
        if procedure.is_some() {
            actual_procedure = procedure.unwrap();
        } else {
            actual_procedure = default_proc;
        }

        self.windows.push(Window {
            hwnd,
            class,
            caption,
            styles,
            rect,
            parent,
            procedure: actual_procedure,
        });

        actual_procedure(hwnd, Message::Create);

        Some(hwnd)
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
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            canvas.present();
        }

        for window in &self.windows {
            (window.procedure)(window.hwnd, Message::Destroy);
        }
    }
}
