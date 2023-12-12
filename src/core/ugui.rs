extern crate sdl2;
use crate::core::geo::Alignment;
use crate::core::geo::{Point, Rect};
use crate::core::messages::Message;
use crate::core::styles::Styles;
use crate::window::Window;
use crate::HWND;
use crate::WNDPROC;
use flagset::FlagSet;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::collections::HashMap;
use std::path::Path;
use crate::CENTER_SCREEN;

/// The global application context, roughly equivalent to a WinAPI INSTANCE
#[derive(Default)]
pub struct Ugui {
    windows: Vec<Window>,
    canvas: Option<WindowCanvas>,
    message_queue: Vec<(HWND, Message)>,
}
impl Ugui {
    fn window_at_point(windows: &[Window], point: Point) -> Option<&Window> {
        if let Some(control) = windows.iter().rev().find(|x| point.inside(x.rect)) {
            return Some(control);
        }
        return None;
    }
}
impl Ugui {
    pub fn paint_quad(
        &mut self,
        rect: Rect,
        back_color: Color,
        border_color: Color,
        border_size: f32,
    ) {
        self.canvas.as_mut().unwrap().set_draw_color(border_color);
        self.canvas
            .as_mut()
            .unwrap()
            .fill_rect(rect.to_sdl())
            .unwrap();
        self.canvas.as_mut().unwrap().set_draw_color(back_color);
        self.canvas
            .as_mut()
            .unwrap()
            .fill_rect(rect.inflate(-1.0).to_sdl())
            .unwrap();
    }
    pub fn paint_text<'a>(
        &mut self,
        font: Font<'a, 'static>,
        text: &str,
        rect: Rect,
        color: Color,
        horizontal_alignment: Alignment,
        vertical_alignment: Alignment,
        line_height: f32,
    ) {
        let texture_creator = self.canvas.as_mut().unwrap().texture_creator();

        let lines = text.split("\n").collect::<Vec<&str>>();

        for i in 0..lines.len() {
            let mut line = lines[i].replace("\n", "");

            // SDL freaks out when performing operations on 0-width strings
            if line.len() == 0 {
                line = " ".to_string();
            }

            let surface = font
                .render(&line)
                .blended(color)
                .map_err(|e| e.to_string())
                .unwrap();

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())
                .unwrap();

            let size = font.size_of(&line).unwrap();
            let text_size = Point {
                x: size.0 as f32,
                y: size.1 as f32,
            };
            let mut line_rect = Rect {
                x: rect.x,
                y: rect.y + (i as f32 * line_height),
                w: rect.w,
                h: line_height,
            };
            if lines.len() == 1 {
                // Single-line string: line rect is just the regular rect
                line_rect = rect;
            }
            if horizontal_alignment == Alignment::Center {
                line_rect.x += line_rect.w / 2.0 - text_size.x / 2.0;
            }
            if horizontal_alignment == Alignment::End {
                line_rect.x += line_rect.w - text_size.x;
            }
            if vertical_alignment == Alignment::Center {
                line_rect.y += line_rect.h / 2.0 - text_size.y / 2.0;
            }
            if vertical_alignment == Alignment::End {
                line_rect.y += line_rect.h - text_size.y;
            }
            line_rect.w = text_size.x;
            line_rect.h = text_size.y;
            self.canvas
                .as_mut()
                .unwrap()
                .copy(&texture, None, Some(line_rect.to_sdl()))
                .unwrap();
        }
    }
}
impl Ugui {
    /// Creates a window with the specified arguments
    ///
    /// # Arguments
    ///
    /// * `class`: (TODO) The control's class name, which is used for determining the window's type
    /// * `caption`: The control's caption, which should consist of descriptive text
    /// * `styles`: Bitfield with various style flags
    /// * `rect`: The control's visual bounds
    /// * `parent`: The window's parent (e.g.: the top-level window, if the created control is a child of it) or `None`
    /// * `procedure`: The window's procedure, which processes messages, or `None`
    ///
    /// returns: Option<HWND> The window's handle
    pub fn create_window(
        &mut self,
        class: String,
        caption: String,
        styles: FlagSet<Styles>,
        rect: Rect,
        parent: Option<HWND>,
        procedure: WNDPROC,
    ) -> Option<HWND> {
        let hwnd = self.windows.len();

        self.windows.push(Window {
            hwnd,
            class,
            caption,
            styles,
            rect,
            parent,
            procedure,
            state_0: 0,
        });

        self.message_queue.push((hwnd, Message::StylesChanged));
        self.message_queue.push((hwnd, Message::Create));

        Some(hwnd)
    }

    /// Destroys a window, notifying it prior to the destruction
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: ()
    pub fn destroy_window(&mut self, hwnd: HWND) {
        let window = &self.windows[hwnd];
        self.send_message(hwnd, Message::Destroy);
        self.windows.remove(hwnd);
    }

    /// Gets a window's styles
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: FlagSet<Styles> The window's styles
    ///
    pub fn get_window_style(&self, hwnd: HWND) -> FlagSet<Styles> {
        self.windows[hwnd].styles
    }

    /// Sets a window's styles and notifies it about the changes
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    /// * `styles`: The styles
    ///
    /// returns: ()
    pub fn set_window_style(&mut self, hwnd: HWND, styles: FlagSet<Styles>) {
        self.windows[hwnd].styles = styles;
        self.send_message(hwnd, Message::StylesChanged);
    }

    /// Gets a window's bounds
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: Rect The window's bounds relative to the top-left of the top-level window
    ///
    pub fn get_window_rect(&self, hwnd: HWND) -> Rect {
        self.windows[hwnd].rect
    }

    /// Sends a message to the specified window and processes it immediately
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    /// * `message`: The message to send
    ///
    /// returns: u64 The window's procedure response
    ///
    pub fn send_message(&mut self, hwnd: HWND, message: Message) -> u64 {
        (self.windows[hwnd].procedure)(self, hwnd, message)
    }

    /// Gets the window's parent
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: Option<HWND> The window's parent, or None if the window is a top-level window
    ///
    pub fn get_parent(&self, hwnd: HWND) -> Option<HWND> {
        self.windows[hwnd].parent
    }

    /// Gets the window's children
    /// All children, even those multiple layers deep are retrieved by this function
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: Vec<HWND> A vector containing the window's children
    ///
    pub fn get_children(&self, hwnd: HWND) -> Vec<HWND> {
        let mut children: Vec<HWND> = vec![];

        for window in &self.windows {
            let mut current_hwnd = window.hwnd;
            while true {
                if self.windows[current_hwnd].parent.is_none() {
                    break;
                }
                if self.windows[current_hwnd].parent.unwrap() == window.hwnd {
                    children.push(current_hwnd);
                    break;
                }
                current_hwnd = self.windows[current_hwnd].parent.unwrap()
            }
        }

        children
    }

    /// Gets a window's styles
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: FlagSet<Styles> A bitfield of styles
    ///
    pub fn get_styles(&self, hwnd: HWND) -> FlagSet<Styles> {
        self.windows[hwnd].styles
    }

    /// Gets a window's user data
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: u64 The user data associated with the window
    pub fn get_udata(&self, hwnd: HWND) -> u64 {
        self.windows[hwnd].state_0
    }

    /// Sets a window's user data
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    /// * `state`: The desired user data
    ///
    pub fn set_udata(&mut self, hwnd: HWND, state: u64) {
        self.windows[hwnd].state_0 = state
    }

    /// Shows a window, trapping the caller until the window closes
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// let hwnd = Ugui::default();
    /// let hwnd = ugui.create_window();
    /// ugui.show_window(hwnd);
    /// ```
    pub fn show_window(&mut self, hwnd: HWND) {
        let window = &mut self.windows[hwnd];

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let ttf_context = sdl2::ttf::init().unwrap();

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

        let mut sdl_window = window_builder.build().unwrap();
        self.canvas = Some(sdl_window.into_canvas().build().unwrap());
        let mut event_pump = sdl_context.event_pump().unwrap();

        let default_font = Some(
            ttf_context
                .load_font(Path::new("../../src/skin/segoe.ttf"), 16)
                .unwrap(),
        );

        let mut lmb_down_point = Point::default();
        let mut last_mouse_position = Point::default();
        let mut focused_hwnd: Option<HWND> = None;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::MouseButtonDown {
                        mouse_btn, x, y, ..
                    } => {
                        let point = Point::new_i(x, y);
                        if mouse_btn != MouseButton::Left {
                            break;
                        }
                        lmb_down_point = point;

                        if let Some(control) = Self::window_at_point(&self.windows, lmb_down_point)
                        {
                            // If focused HWNDs differ, we unfocus the old one
                            if focused_hwnd.is_some() && focused_hwnd.unwrap() != control.hwnd {
                                self.message_queue.push((control.hwnd, Message::Unfocus));
                            }

                            let prev_focused_hwnd = focused_hwnd;
                            focused_hwnd = Some(control.hwnd);
                            self.message_queue.push((control.hwnd, Message::LmbDown));

                            // Only send focus message if focus state actually changes after reassignment
                            if focused_hwnd.ne(&prev_focused_hwnd) {
                                self.message_queue.push((control.hwnd, Message::Focus));
                            }
                        }
                    }
                    Event::MouseButtonUp {
                        mouse_btn, x, y, ..
                    } => {
                        let point = Point::new_i(x, y);
                        if mouse_btn != MouseButton::Left {
                            break;
                        }
                        // Tell the previously clicked control we left it now
                        if let Some(control) = Self::window_at_point(&self.windows, lmb_down_point)
                        {
                            self.message_queue.push((control.hwnd, Message::LmbUp));
                        }
                    }
                    Event::MouseMotion {
                        x, y, mousestate, ..
                    } => {
                        let point = Point::new_i(x, y);

                        // If we have a control at the mouse, we send it mousemove
                        if let Some(control) = Self::window_at_point(&self.windows, lmb_down_point)
                        {
                            self.message_queue.push((
                                control.hwnd,
                                Message::MouseMove(point.sub(control.rect.top_left())),
                            ));
                        }

                        if let Some(control) = Self::window_at_point(&self.windows, point) {
                            if let Some(prev_control) =
                                Self::window_at_point(&self.windows, last_mouse_position)
                            {
                                if control.hwnd != prev_control.hwnd {
                                    self.message_queue.push((control.hwnd, Message::MouseEnter));
                                    self.message_queue
                                        .push((prev_control.hwnd, Message::MouseLeave));
                                }
                            }
                        }

                        last_mouse_position = point;
                    }
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::SizeChanged(w, h) => {
                            // Update this top-level window's dimensions
                            self.windows[hwnd].rect.w = w as f32;
                            self.windows[hwnd].rect.h = h as f32;

                            for window in &self.windows {
                                self.message_queue.push((window.hwnd, Message::Paint));
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            for i in 0..self.message_queue.len() {
                let wnd = &self.windows[self.message_queue[i].0];
                (wnd.procedure)(self, wnd.hwnd, self.message_queue[i].1);
            }

            self.message_queue.clear();
            self.canvas.as_mut().unwrap().present();
        }

        for i in 0..self.windows.len() {
            self.destroy_window(self.windows[i].hwnd);
        }
    }
}
