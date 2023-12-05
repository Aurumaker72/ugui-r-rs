extern crate sdl2;

use crate::controls::control::{BaseControl, Control};
use crate::core::geo::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use sdl2::ttf::Sdl2TtfContext;
use sdl2::EventPump;

use std::path::Path;

pub struct WindowBuilder {
    title: String,
    w: u32,
    h: u32,
    content: Control,
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            title: "ugui-r-rs".to_string(),
            w: 640,
            h: 480,
            content: Control::Label {
                base: BaseControl::default(),
                text: "Hello World!".to_string(),
            },
        }
    }
    pub fn title(mut self, title: String) -> WindowBuilder {
        self.title = title;
        self
    }
    pub fn width(mut self, w: u32) -> WindowBuilder {
        self.w = w;
        self
    }
    pub fn height(mut self, h: u32) -> WindowBuilder {
        self.h = h;
        self
    }
    pub fn content(mut self, control: Control) -> WindowBuilder {
        self.content = control;
        self
    }
    pub fn build(self) -> Window {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

        let window = video_subsystem
            .window(&self.title, self.w, self.h)
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
            content: self.content,
            ttf_context,
        }
    }
}

pub struct Window {
    canvas: WindowCanvas,
    event_pump: EventPump,
    content: Control,
    ttf_context: Sdl2TtfContext,
}

impl Window {
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

            self.content.render(&mut self.canvas);
            self.canvas.present();
        }
    }

    pub fn set_content(&mut self, control: Control) {
        self.content = control;
        // FIXME: Invalidate layout!
    }
}
