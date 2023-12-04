extern crate sdl2;

use crate::controls::control::BaseControl;
use crate::controls::control::{get_base, Control};
use crate::core::layout::compute_layout_bounds;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Window {
    canvas: WindowCanvas,
    event_pump: EventPump,
    content: Control,
}

impl Window {
    pub fn new() -> Window {
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
        }
    }

    pub fn show(&mut self) {
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

            let controls = get_base(self.content.clone()).unwrap().get_children();

            // If a control is invalidated, its children need to be invalidated too
            for control in controls {
                let base = get_base(control).unwrap();
                if !base.validated {
                    base.get_children()
                        .iter()
                        .map(|x| get_base(x.clone()).unwrap())
                        .for_each(|mut x| x.validated = false);
                }
            }
            //
            // // Recompute layout bounds of all invalidated controls, then validate them
            // for control in controls {
            //     let base = get_base(control).unwrap();
            //
            //     if !base.validated {
            //         base.computed_bounds = compute_layout_bounds(self, &control);
            //         base.validated = true;
            //         println!("Validated!");
            //     }
            // }

            self.canvas.present();
        }
    }

    pub fn set_content(&mut self, control: Control) {
        self.content = control;
    }
}
