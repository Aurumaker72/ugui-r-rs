extern crate sdl2;
use crate::core::geo::{Point, Rect};
use crate::core::messages::Message;
use crate::core::styles::Styles;
use flagset::FlagSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::collections::HashMap;
use std::path::Path;

pub type HWND = usize;
pub type WNDPROC = fn(&mut Ugui, HWND, Message) -> u64;
pub const CENTER_SCREEN: f32 = -1.0;

#[derive(Clone)]
struct Window {
    hwnd: HWND,
    class: String,
    caption: String,
    styles: FlagSet<Styles>,
    rect: Rect,
    parent: Option<HWND>,
    procedure: WNDPROC,
}

impl Window {
    fn top_level(&self) -> bool {
        self.parent.is_none()
    }
}

pub fn default_proc(ugui: &mut Ugui, hwnd: HWND, message: Message) -> u64 {
    println!("{} {:?}", hwnd, message);

    match message {
        Message::Create => {
            ugui.invalidate_window(hwnd);
        }
        Message::Paint => {
            let rect = ugui.get_window_rect(hwnd);
            ugui.paint_quad(rect, Color::RGB(255, 0, 0), Color::RGB(255, 55, 55), 1.0);
        }
        _ => {}
    }
    0
}

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
        });

        self.message_queue.push((hwnd, Message::Create));
        self.message_queue
            .push((hwnd, Message::StylesChanged(styles)));

        Some(hwnd)
    }

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

    /// Destroys a window, notifying it prior to the destruction
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: ()
    pub fn destroy_window(&mut self, hwnd: HWND) {
        let window = &self.windows[hwnd];
        (window.procedure)(self, window.hwnd, Message::Destroy);
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
        (self.windows[hwnd].procedure)(
            self,
            self.windows[hwnd].hwnd,
            Message::StylesChanged(styles),
        );
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

    /// Invalidates a window, queueing a paint message
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: ()
    ///
    pub fn invalidate_window(&mut self, hwnd: HWND) {
        self.message_queue.push((hwnd, Message::Paint));
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
        let window = &self.windows[hwnd];
        let mut lmb_down_point = Point::default();
        let mut focused_hwnd: Option<HWND> = None;

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

        let mut window = window_builder.build().unwrap();

        self.canvas = Some(window.into_canvas().build().unwrap());
        let mut event_pump = sdl_context.event_pump().unwrap();

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
                    }
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
