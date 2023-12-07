extern crate sdl2;
use crate::core::geo::{Point, Rect};
use crate::core::messages::Message;
use crate::core::styles::Styles;
use flagset::FlagSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
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
    println!("{} {:?}", hwnd, message);
    return 0;
}

/// The global application context, roughly equivalent to a WinAPI INSTANCE
#[derive(Default)]
pub struct Ugui {
    windows: Vec<Window>,
}

impl Ugui {
    fn window_at_point(&self, point: Point) -> Option<&Window> {
        self.windows.iter().find(|x| point.inside(x.rect))
    }
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
        let mut lmb_down_point = Point::default();
        let mut focused_hwnd: Option<HWND> = None;

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
                    Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => {
                        let point = Point::new_i(x, y);

                        if mouse_btn == MouseButton::Left {
                            lmb_down_point = point;
                            if let Some(control) = self.window_at_point(lmb_down_point) {
                                // If focused HWNDs differ, we unfocus the old one
                                if focused_hwnd.is_some() && focused_hwnd.unwrap() != control.hwnd {
                                    (control.procedure)(control.hwnd, Message::Unfocus);
                                }

                                focused_hwnd = Some(control.hwnd);
                                (control.procedure)(control.hwnd, Message::LmbDown);

                                (control.procedure)(control.hwnd, Message::Focus);
                            }
                        }
                    }
                    Event::MouseButtonUp {
                        mouse_btn, x, y, ..
                    } => {
                        let point = Point::new_i(x, y);

                        if mouse_btn == MouseButton::Left {
                            // Tell the previously clicked control we left it now
                            if let Some(control) = self.window_at_point(lmb_down_point) {
                                (control.procedure)(control.hwnd, Message::LmbUp);
                            }
                        }
                    }
                    Event::MouseMotion { x, y, .. } => {}
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
