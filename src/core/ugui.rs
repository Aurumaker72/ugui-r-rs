extern crate sdl2;
use crate::core::geo::Alignment;
use crate::core::geo::{Point, Rect};
use crate::core::messages::Message;
use crate::core::styles::Styles;
use crate::window::Window;
use crate::CENTER_SCREEN;
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

/// An application, roughly equivalent to a top-level window with a message loop and many child windows.
#[derive(Default)]
pub struct Ugui {
    windows: Vec<Window>,
    canvas: Option<WindowCanvas>,
    message_queue: Vec<(HWND, Message)>,
    captured_hwnd: Option<HWND>,
    /// Whether the buffers need to be swapped. It's expensive to swap buffers, so we only do this when receiving paint events
    needs_swap: bool,
}

impl Ugui {
    fn window_at_point(windows: &[Window], point: Point) -> Option<&Window> {
        if let Some(control) = windows.iter().rev().find(|x| point.inside(x.rect)) {
            return Some(control);
        }
        return None;
    }

    fn window_from_hwnd(windows: &[Window], hwnd: HWND) -> &Window {
        if let Some(window) = windows.iter().find(|x| x.hwnd == hwnd) {
            return window;
        }
        panic!("No window with specified HWND found");
    }
    fn window_from_hwnd_mut(windows: &mut [Window], hwnd: HWND) -> &mut Window {
        for i in 0..windows.len() {
            if windows[i].hwnd == hwnd {
                return &mut windows[i];
            }
        }
        panic!("No window with specified HWND found");
    }
    fn root_hwnd(&self) -> HWND {
        // NOTE: This is guaranteed
        self.windows[0].hwnd
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
        self.message_queue.push((hwnd, Message::Paint));

        Some(hwnd)
    }

    /// Destroys a window, notifying it prior to the destruction
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    pub fn destroy_window(&mut self, hwnd: HWND) {
        let window = Ugui::window_from_hwnd(&self.windows, hwnd);
        self.send_message(window.hwnd, Message::Destroy);
        self.windows.iter().filter(|x| x.hwnd != hwnd);
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
        Ugui::window_from_hwnd(&self.windows, hwnd).styles
    }

    /// Sets a window's styles and notifies it about the changes
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    /// * `styles`: The styles
    ///
    pub fn set_window_style(&mut self, hwnd: HWND, styles: FlagSet<Styles>) {
        let window = Ugui::window_from_hwnd_mut(&mut self.windows, hwnd);
        window.styles = styles;
        self.send_message(hwnd, Message::StylesChanged);
        self.send_message(hwnd, Message::Paint);
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
        Ugui::window_from_hwnd(&self.windows, hwnd).rect
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
        if message == Message::Paint {
            self.needs_swap = true;
        }
        (Ugui::window_from_hwnd(&self.windows, hwnd).procedure)(self, self.root_hwnd(), hwnd, message)
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
        Ugui::window_from_hwnd(&self.windows, hwnd).parent
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
                let current_window = Ugui::window_from_hwnd(&self.windows, current_hwnd);
                if current_window.parent.is_none() {
                    break;
                }
                if current_window.parent.unwrap() == window.hwnd {
                    children.push(current_hwnd);
                    break;
                }
                current_hwnd = current_window.parent.unwrap()
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
        Ugui::window_from_hwnd(&self.windows, hwnd).styles
    }

    /// Gets a window's user data
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: u64 The user data associated with the window
    pub fn get_udata(&self, hwnd: HWND) -> u64 {
        Ugui::window_from_hwnd(&self.windows, hwnd).state_0
    }

    /// Sets a window's user data
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    /// * `state`: The desired user data
    ///
    pub fn set_udata(&mut self, hwnd: HWND, state: u64) {
        Ugui::window_from_hwnd_mut(&mut self.windows, hwnd).state_0 = state
    }

    /// Captures the mouse, receiving all of its events and preventing propagation to other controls
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window handle capturing the mouse
    pub fn capture_mouse(&mut self, hwnd: HWND) {
        self.captured_hwnd = Some(hwnd)
    }

    /// Release mouse capture, resuming normal event propagation
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window handle releasing the mouse capture
    pub fn uncapture_mouse(&mut self, hwnd: HWND) {
        self.captured_hwnd = None
    }

    /// Paints a decorated quad
    ///
    /// # Arguments
    ///
    /// * `rect`: The quad destination
    /// * `back_color`: The quad's background color
    /// * `border_color`: The quad's border color
    /// * `border_size`: The quad's border size
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

    /// Paints unformatted text
    ///
    /// # Arguments
    ///
    /// * `font`: The font to paint the text with
    /// * `text`: The text
    /// * `rect`: The text bounds
    /// * `color`: The text color
    /// * `horizontal_alignment`: The text's horizontal alignment inside its bounds
    /// * `vertical_alignment`: The text's vertical alignment inside its bounds
    /// * `line_height`: The space between line breaks
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
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let ttf_context = sdl2::ttf::init().unwrap();

        let top_level_window = Ugui::window_from_hwnd(&self.windows, hwnd);
        let mut window_builder = &mut video_subsystem.window(
            &top_level_window.caption,
            top_level_window.rect.w as u32,
            top_level_window.rect.h as u32,
        );

        window_builder
            .position(
                top_level_window.rect.x as i32,
                top_level_window.rect.y as i32,
            )
            .opengl()
            .resizable();

        if top_level_window.rect.x == CENTER_SCREEN && top_level_window.rect.y == CENTER_SCREEN {
            window_builder = window_builder.position_centered();
        }

        let mut sdl_window = window_builder.build().unwrap();
        self.canvas = Some(sdl_window.into_canvas().present_vsync().build().unwrap());
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
            let mouse_point =
                Point::new_i(event_pump.mouse_state().x(), event_pump.mouse_state().y());

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        if mouse_btn != MouseButton::Left {
                            break;
                        }
                        lmb_down_point = mouse_point;

                        if let Some(control) = Self::window_at_point(&self.windows, lmb_down_point)
                        {
                            // If focused HWNDs differ, we unfocus the old one
                            if focused_hwnd.is_some() && focused_hwnd.unwrap() != control.hwnd {
                                self.message_queue
                                    .push((focused_hwnd.unwrap(), Message::Unfocus));
                            }

                            focused_hwnd = Some(control.hwnd);
                            self.message_queue.push((control.hwnd, Message::LmbDown));
                            self.message_queue.push((control.hwnd, Message::Focus));
                        }
                    }
                    Event::MouseButtonUp { mouse_btn, .. } => {
                        if mouse_btn != MouseButton::Left {
                            break;
                        }
                        // Following assumption is made: We can't have up without down happening prior to it.
                        // The control at the mouse down position thus needs to know if the mouse was released afterwards, either inside or outside of its client area.
                        if let Some(control) = Self::window_at_point(&self.windows, lmb_down_point)
                        {
                            self.message_queue.push((control.hwnd, Message::LmbUp));
                        }
                    }
                    Event::MouseMotion { .. } => {
                        // If we have a captured control, it gets special treatment
                        if let Some(captured_hwnd) = self.captured_hwnd {
                            let captured_window =
                                Ugui::window_from_hwnd(&self.windows, captured_hwnd);
                            // 1. Send MouseMove unconditionally
                            self.message_queue.push((captured_hwnd, Message::MouseMove));

                            // 2. Send MouseEnter/Leave based solely off of its own client rect
                            if mouse_point.inside(captured_window.rect)
                                && !last_mouse_position.inside(captured_window.rect)
                            {
                                self.message_queue
                                    .push((captured_hwnd, Message::MouseEnter));
                            }
                            if !mouse_point.inside(captured_window.rect)
                                && last_mouse_position.inside(captured_window.rect)
                            {
                                self.message_queue
                                    .push((captured_hwnd, Message::MouseLeave));
                            }
                        } else {
                            if let Some(control) = Self::window_at_point(&self.windows, mouse_point)
                            {
                                // We have no captured control, so it's safe to regularly send MouseMove to the window under the mouse
                                self.message_queue.push((control.hwnd, Message::MouseMove));

                                if let Some(prev_control) =
                                    Self::window_at_point(&self.windows, last_mouse_position)
                                {
                                    if control.hwnd != prev_control.hwnd {
                                        self.message_queue
                                            .push((control.hwnd, Message::MouseEnter));
                                        self.message_queue
                                            .push((prev_control.hwnd, Message::MouseLeave));
                                    }
                                }
                            }
                        }
                        last_mouse_position = mouse_point;
                    }
                    Event::Window { win_event, .. } => match win_event {
                        WindowEvent::SizeChanged(w, h) => {
                            // Update this top-level window's dimensions
                            let top_level_window =
                                Ugui::window_from_hwnd_mut(&mut self.windows, hwnd);
                            top_level_window.rect.w = w as f32;
                            top_level_window.rect.h = h as f32;

                            for window in &self.windows {
                                self.message_queue.push((window.hwnd, Message::Paint));
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            for (hwnd, message) in self.message_queue.clone() {
                self.send_message(hwnd, message);
            }

            self.message_queue.clear();

            if self.needs_swap {
                self.canvas.as_mut().unwrap().present();
                self.needs_swap = false;
            }
        }

        for window in self.windows.clone().iter().rev() {
            self.destroy_window(window.hwnd);
        }
    }
}
