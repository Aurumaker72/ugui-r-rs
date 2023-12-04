extern crate sdl2;

use crate::controls::control::BaseControl;
use crate::controls::control::Control;
use crate::core::geo::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::EventPump;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::rc::Rc;

pub struct Window {
    canvas: WindowCanvas,
    event_pump: EventPump,
    content: Control,
    ttf_context: Sdl2TtfContext,
}

impl Window {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        let window = video_subsystem
            .window("Test", 640, 480)
            .opengl()
            .resizable()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string()).unwrap();

        Window {
            event_pump,
            canvas,
            content: Control::None,
            ttf_context,
        }
    }

    pub fn show(&mut self) {
        // TODO: fix this magic path bullshit
        let font = self
            .ttf_context
            .load_font(Path::new("../../src/skin/segoe.ttf"), 16)
            .unwrap();

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.content.do_layout(
                Rect::new(
                    0.0,
                    0.0,
                    self.canvas.window().drawable_size().0 as f32,
                    self.canvas.window().drawable_size().1 as f32,
                ),
                &font,
            );

            self.canvas.present();
        }
    }

    pub fn set_content(&mut self, control: Control) {
        self.content = control;
        self.content.get_base_mut().validated = false;
    }
}
