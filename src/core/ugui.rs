use crate::core::dynval::Value;
use crate::core::messages::Message;
use crate::core::util::*;
use crate::gfx::alignment::Alignment;
use crate::gfx::color::Color;
use crate::gfx::drawop::{QuadDrawOp, TextDrawOp};
use crate::gfx::point::Point;
use crate::gfx::rect::Rect;
use crate::gfx::styles::Styles;
use crate::input::mouse::MouseState;
use crate::window::Window;
use crate::HWND;
use crate::WNDPROC;
use flagset::FlagSet;

use crate::gfx::painter::Painter;
use ggez::event::winit_event::{ElementState, Event, KeyboardInput, WindowEvent};
use ggez::event::{ControlFlow, MouseButton};
use ggez::glam::Vec2;
use ggez::graphics::{
    self, Canvas, DrawMode, FillOptions, PxScale, Text, TextAlign, TextFragment, TextLayout,
};
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::GameResult;
use ggez::{event, mint, Context};

/// An application, roughly equivalent to a top-level window with a message loop and many child windows.
#[derive(Default)]
pub struct Ugui {
    windows: Vec<Window>,
    message_queue: Vec<(HWND, Message)>,
    captured_hwnd: Option<HWND>,
    focused_hwnd: Option<HWND>,
    /// The last text inputted by the user via keyboard
    /// We need to store this as an intermediary here, since it would have to be squeezed into a Message otherwise, thus making it non-copyable
    last_text_input: String,
    /// Regions relative to top-level window which need to be repainted
    dirty_rects: Vec<Rect>,
    /// Wrapper for mouse state, can be retrieved by user
    mouse_state: MouseState,
    /// A list of quad drawing operations
    quad_draw_ops: Vec<QuadDrawOp>,
    /// A list of text drawing operations
    text_draw_ops: Vec<TextDrawOp>,
    /// A stack of clip rectangles
    clip_rects: Vec<Rect>,
}
impl Painter for Ugui {
    fn paint_quad(&mut self, rect: Rect, back_color: Color, border_color: Color, border_size: f32) {
        self.quad_draw_ops.push(QuadDrawOp {
            rect,
            clip_rect: self.clip_rects.last().copied(),
            back_color,
            border_color,
            border_size,
        });
    }

    fn paint_text<'a>(
        &mut self,
        text: &str,
        rect: Rect,
        color: Color,
        horizontal_alignment: Alignment,
        vertical_alignment: Alignment,
        size: f32,
        line_height: f32,
    ) {
        self.text_draw_ops.push(TextDrawOp {
            color,
            text: text.to_string(),
            rect,
            clip_rect: self.clip_rects.last().copied(),
            h_align: horizontal_alignment,
            v_align: vertical_alignment,
            size,
            line_height,
        });
    }
    fn push_clip(&mut self, rect: Rect) {
        self.clip_rects.push(rect);
    }

    fn pop_clip(&mut self) {
        self.clip_rects.pop();
    }
}

impl Ugui {
    /// Paints all controls inside a rectangle
    fn paint_rect(&mut self, rect: Rect) {
        // 1. Clip rendering to control bounds
        self.push_clip(rect);

        // 2. Repaint all controls inside the affected rect, skipping invisible ones
        let affected_windows = get_window_handles_inside_rect(&self.windows, rect);

        for hwnd in affected_windows {
            let window = window_from_hwnd(&self.windows, hwnd);

            if !window.styles.contains(Styles::Visible) {
                continue;
            }
            (window.procedure)(self, hwnd, Message::Paint);
        }

        // 3. Pop clip
        self.pop_clip();
    }

    fn process_dirty_rects(&mut self) {
        for rect in self.dirty_rects.clone() {
            self.paint_rect(rect);
        }
        self.dirty_rects.clear();
    }
    fn process_draw_ops(&mut self, ctx: &mut Context) {
        let mut canvas = Canvas::from_frame(ctx, None);

        for quad_draw_op in &self.quad_draw_ops {
            if let Some(clip_rect) = quad_draw_op.clip_rect {
                canvas.set_scissor_rect(graphics::Rect::new(
                    clip_rect.x,
                    clip_rect.y,
                    clip_rect.w,
                    clip_rect.h,
                ));
            } else {
                canvas.set_default_scissor_rect();
            }
            let back_rect = graphics::Rect::new(
                quad_draw_op.rect.x + quad_draw_op.border_size,
                quad_draw_op.rect.y + quad_draw_op.border_size,
                quad_draw_op.rect.w - quad_draw_op.border_size * 2.0,
                quad_draw_op.rect.h - quad_draw_op.border_size * 2.0,
            );
            let border_rect = graphics::Rect::new(
                quad_draw_op.rect.x,
                quad_draw_op.rect.y,
                quad_draw_op.rect.w,
                quad_draw_op.rect.h,
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(border_rect.point())
                    .scale(border_rect.size())
                    .color(graphics::Color::from_rgba(
                        quad_draw_op.border_color.r,
                        quad_draw_op.border_color.g,
                        quad_draw_op.border_color.b,
                        quad_draw_op.border_color.a,
                    )),
            );
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(back_rect.point())
                    .scale(back_rect.size())
                    .color(graphics::Color::from_rgba(
                        quad_draw_op.back_color.r,
                        quad_draw_op.back_color.g,
                        quad_draw_op.back_color.b,
                        quad_draw_op.back_color.a,
                    )),
            );
        }
        self.quad_draw_ops.clear();

        for text_draw_op in self.text_draw_ops.clone() {
            if let Some(clip_rect) = text_draw_op.clip_rect {
                canvas.set_scissor_rect(graphics::Rect::new(
                    clip_rect.x,
                    clip_rect.y,
                    clip_rect.w,
                    clip_rect.h,
                ));
            } else {
                canvas.set_default_scissor_rect();
            }
            let mut text = Text::new(TextFragment {
                text: text_draw_op.text,
                color: Some(graphics::Color::from_rgba(
                    text_draw_op.color.r,
                    text_draw_op.color.g,
                    text_draw_op.color.b,
                    text_draw_op.color.a,
                )),
                scale: Some(PxScale::from(text_draw_op.size)),
                ..Default::default()
            });
            text.set_bounds(Vec2::new(text_draw_op.rect.w, text_draw_op.rect.h))
                .set_layout(TextLayout {
                    h_align: TextAlign::Begin,
                    v_align: TextAlign::Begin,
                });

            let size = text.measure(ctx).unwrap();

            let x = match text_draw_op.h_align {
                Alignment::Start => text_draw_op.rect.x,
                Alignment::End => text_draw_op.rect.right() - size.x,
                _ => text_draw_op.rect.x + text_draw_op.rect.w / 2.0 - size.x / 2.0,
            };
            let y = match text_draw_op.v_align {
                Alignment::Start => text_draw_op.rect.y,
                Alignment::End => text_draw_op.rect.bottom() - size.y,
                _ => text_draw_op.rect.y + text_draw_op.rect.h / 2.0 - size.y / 2.0,
            };
            canvas.draw(
                &text,
                graphics::DrawParam::from([x, y]).color(graphics::Color::WHITE),
            );
        }
        self.text_draw_ops.clear();

        canvas.finish(ctx).unwrap();
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
            user_data: None,
        });

        // NOTE: We need to send and process these NOW, as this method could be called from inside a WndProc
        // In that case, we would be performing the repaint immediately after processing the messages
        // This leaves the control in an invalid state, since it was painted without being created first
        self.send_message(hwnd, Message::StylesChanged);
        self.send_message(hwnd, Message::Create);
        self.invalidate_rect(rect);

        Some(hwnd)
    }

    /// Gets the current mouse state
    pub fn mouse_state(&self) -> MouseState {
        self.mouse_state
    }

    /// Gets the focused window
    pub fn get_focus(&self) -> Option<HWND> {
        self.focused_hwnd
    }

    /// Gets the captured window
    pub fn get_capture(&self) -> Option<HWND> {
        self.captured_hwnd
    }

    /// Invalidates a rectangle, marking all controls inside it to receive Paint message and be composited eventually
    ///
    /// # Arguments
    ///
    /// * `rect`: The bounds to invalidate
    pub fn invalidate_rect(&mut self, rect: Rect) {
        self.dirty_rects.push(rect);
    }

    /// Invalidates a control's rectangle, marking all controls inside it to receive Paint message and be composited eventually
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    pub fn invalidate_hwnd(&mut self, hwnd: HWND) {
        self.dirty_rects
            .push(window_from_hwnd(&self.windows, hwnd).rect);
    }

    /// Gets the most recent typed text
    pub fn typed_text(&self) -> &str {
        self.last_text_input.as_str()
    }

    /// Gets the top-level window's root handle
    pub fn root_hwnd(&self) -> HWND {
        // NOTE: This is guaranteed
        self.windows[0].hwnd
    }

    /// Destroys a window, removing it from the hierarchy and notifying it
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    pub fn destroy_window(&mut self, hwnd: HWND) {
        // HACK: We set the control's invisible flag, and then force a rect-region repaint, so it disappears
        // We can't use normal repaint message since that assumes a valid control exists
        self.set_window_style(hwnd, Styles::None.into());
        self.paint_rect(self.get_window_rect(hwnd));

        self.send_message(hwnd, Message::Destroy);
        self.windows.retain(|x| x.hwnd != hwnd);

        self.captured_hwnd = fix_dependent_visual_handle(&self.windows, self.captured_hwnd);
        self.focused_hwnd = fix_dependent_visual_handle(&self.windows, self.focused_hwnd);
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
        window_from_hwnd(&self.windows, hwnd).styles
    }

    /// Sets a window's styles and notifies it about the changes
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    /// * `styles`: The styles
    ///
    pub fn set_window_style(&mut self, hwnd: HWND, styles: FlagSet<Styles>) {
        let window = window_from_hwnd_mut(&mut self.windows, hwnd);
        let rect = window.rect;
        window.styles = styles;

        self.captured_hwnd = fix_dependent_visual_handle(&self.windows, self.captured_hwnd);
        self.focused_hwnd = fix_dependent_visual_handle(&self.windows, self.focused_hwnd);

        self.send_message(hwnd, Message::StylesChanged);
        self.invalidate_rect(rect);
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
        window_from_hwnd(&self.windows, hwnd).rect
    }

    /// Sets a window's bounds and invalidates its visuals
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    /// * `rect`: The window's bounds
    pub fn set_window_rect(&mut self, hwnd: HWND, rect: Rect) {
        let window = window_from_hwnd_mut(&mut self.windows, hwnd);
        window.rect = rect;
        self.invalidate_hwnd(hwnd);
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
        if window_from_hwnd_safe(&self.windows, hwnd).is_none() {
            println!("Tried to send message to non-existent window");
            return 0;
        }

        return (window_from_hwnd(&self.windows, hwnd).procedure)(self, hwnd, message);
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
        window_from_hwnd(&self.windows, hwnd).parent
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
    pub fn get_children(&self, _hwnd: HWND) -> Vec<HWND> {
        let mut children: Vec<HWND> = vec![];

        for window in &self.windows {
            let mut current_hwnd = window.hwnd;
            loop {
                let current_window = window_from_hwnd(&self.windows, current_hwnd);
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
        window_from_hwnd(&self.windows, hwnd).styles
    }

    /// Gets a window's data
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    ///
    /// returns: Option<Box<dyn Any>> The user data associated with the window
    pub fn get_data(&self, hwnd: HWND) -> Option<Box<dyn Value>> {
        window_from_hwnd(&self.windows, hwnd).user_data.clone()
    }

    /// Sets a window's data
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    /// * `value`: The desired user data
    ///
    pub fn set_data(&mut self, hwnd: HWND, value: Option<Box<dyn Value>>) {
        window_from_hwnd_mut(&mut self.windows, hwnd).user_data = value;
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
    pub fn uncapture_mouse(&mut self, _hwnd: HWND) {
        self.captured_hwnd = None
    }

    /// Gets a window's caption
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    pub fn get_caption(&self, hwnd: HWND) -> String {
        let window = window_from_hwnd(&self.windows, hwnd);
        window.caption.clone()
    }

    /// Sets a window's caption
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    /// * `value`: The new caption
    ///
    pub fn set_caption(&mut self, hwnd: HWND, value: String) {
        let window = window_from_hwnd_mut(&mut self.windows, hwnd);
        window.caption = value;

        // The top-level window gets special treatment: its caption is the title
        if window.parent == None {
            // self.canvas
            //     .as_mut()
            //     .unwrap()
            //     .window_mut()
            //     .set_title(window.caption.as_str());
        } else {
            self.dirty_rects.push(window.rect);
        }
    }

    /// Shows a window, trapping the caller until the window closes
    ///
    /// # Arguments
    ///
    /// * `hwnd`: The window's handle
    pub fn show_window(mut self, hwnd: HWND) {
        let cb = ggez::ContextBuilder::new("eventloop", "ggez");
        let (mut ctx, events_loop) = cb.build().unwrap();
        events_loop.run(move |mut event, _window_target, control_flow| {
            let ctx = &mut ctx;

            if ctx.quit_requested {
                ctx.continuing = false;
            }
            if !ctx.continuing {
                *control_flow = ControlFlow::Exit;
                return;
            }

            *control_flow = ControlFlow::Poll;

            event::process_event(ctx, &mut event);
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => ctx.request_quit(),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    } => {
                        if let keyboard::KeyCode::Escape = keycode {
                            ctx.request_quit();
                        }
                    }
                    WindowEvent::Resized(size) => {
                        // Update this top-level window's dimensions
                        let top_level_window = window_from_hwnd_mut(&mut self.windows, hwnd);
                        top_level_window.rect = Rect {
                            x: 0.0,
                            y: 0.0,
                            w: size.width as f32 + 1.0,
                            h: size.height as f32 + 1.0,
                        };
                        self.invalidate_hwnd(hwnd);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.mouse_state.pos = Point {
                            x: position.x as f32,
                            y: position.y as f32,
                        };

                        // If we have a captured control, it gets special treatment
                        if let Some(captured_hwnd) = self.captured_hwnd {
                            let captured_window = window_from_hwnd(&self.windows, captured_hwnd);
                            // 1. Send MouseMove unconditionally
                            self.message_queue.push((captured_hwnd, Message::MouseMove));
                            // 2. Send MouseEnter/Leave based solely off of its own client rect
                            if self.mouse_state.pos.inside(captured_window.rect)
                                && !self.mouse_state.last_pos.inside(captured_window.rect)
                            {
                                self.message_queue
                                    .push((captured_hwnd, Message::MouseEnter));
                            }
                            if !self.mouse_state.pos.inside(captured_window.rect)
                                && self.mouse_state.last_pos.inside(captured_window.rect)
                            {
                                self.message_queue
                                    .push((captured_hwnd, Message::MouseLeave));
                            }
                        } else {
                            if let Some(control) =
                                window_at_point(&self.windows, self.mouse_state.pos)
                            {
                                // We have no captured control, so it's safe to regularly send MouseMove to the window under the mouse
                                self.message_queue.push((control.hwnd, Message::MouseMove));
                                if let Some(prev_control) =
                                    window_at_point(&self.windows, self.mouse_state.last_pos)
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
                        self.mouse_state.last_pos = self.mouse_state.pos;
                    }
                    WindowEvent::MouseInput { button, state, .. } => match button {
                        MouseButton::Left => {
                            self.mouse_state.lmb = state == ElementState::Pressed;
                            self.mouse_state.lmb_down_pos = self.mouse_state.pos;
                        }
                        MouseButton::Middle => {
                            self.mouse_state.mmb = state == ElementState::Pressed;
                        }
                        MouseButton::Right => {
                            self.mouse_state.rmb = state == ElementState::Pressed;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Event::MainEventsCleared => {
                    ctx.time.tick();
                    ctx.gfx.begin_frame().unwrap();

                    for (hwnd, message) in self.message_queue.clone() {
                        // Paint messages should never arrive in the message queue, since they're always directly sent as part of dirty rect processor
                        assert_ne!(message, Message::Paint);
                        self.send_message(hwnd, message);
                    }
                    self.message_queue.clear();

                    // Repaint all dirty rects, which generates drawing operations
                    self.process_dirty_rects();

                    // Process the drawing operations
                    self.process_draw_ops(ctx);

                    ctx.gfx.end_frame().unwrap();

                    // Update phase
                    ctx.mouse.reset_delta();
                    ctx.keyboard.save_keyboard_state();
                    ctx.mouse.save_mouse_state();

                    ggez::timer::yield_now();
                }

                x => {}
            }
        });

        //
        // 'running: loop {
        //     self.mouse_state.pos =
        //         Point::new_i(event_pump.mouse_state().x(), event_pump.mouse_state().y());
        //
        //     for event in event_pump.poll_iter() {
        //         match event {
        //             Event::Quit { .. } => break 'running,
        //             Event::MouseButtonDown { mouse_btn, .. } => {
        //                 match mouse_btn {
        //                     MouseButton::Left => {
        //                         self.mouse_state.lmb = true;
        //                     }
        //                     MouseButton::Middle => {
        //                         self.mouse_state.mmb = true;
        //                     }
        //                     MouseButton::Right => {
        //                         self.mouse_state.rmb = true;
        //                     }
        //                     _ => {}
        //                 }
        //
        //                 if mouse_btn != MouseButton::Left {
        //                     break;
        //                 }
        //
        //                 self.mouse_state.lmb_down_pos = self.mouse_state.pos;
        //
        //                 if let Some(control) =
        //                     window_at_point(&self.windows, self.mouse_state.lmb_down_pos)
        //                 {
        //                     // If focused HWNDs differ, we unfocus the old one
        //                     if self.focused_hwnd.is_some_and(|x| x != control.hwnd) {
        //                         if window_from_hwnd(&self.windows, self.focused_hwnd.unwrap())
        //                             .styles
        //                             .contains(Styles::Focusable)
        //                         {
        //                             self.message_queue
        //                                 .push((self.focused_hwnd.unwrap(), Message::Unfocus));
        //                         }
        //                     }
        //
        //                     self.focused_hwnd = Some(control.hwnd);
        //                     self.message_queue.push((control.hwnd, Message::LmbDown));
        //
        //                     if window_from_hwnd(&self.windows, control.hwnd)
        //                         .styles
        //                         .contains(Styles::Focusable)
        //                     {
        //                         self.message_queue.push((control.hwnd, Message::Focus));
        //                     }
        //                 }
        //             }
        //             Event::MouseButtonUp { mouse_btn, .. } => {
        //                 match mouse_btn {
        //                     MouseButton::Left => {
        //                         self.mouse_state.lmb = true;
        //                     }
        //                     MouseButton::Middle => {
        //                         self.mouse_state.mmb = true;
        //                     }
        //                     MouseButton::Right => {
        //                         self.mouse_state.rmb = true;
        //                     }
        //                     _ => {}
        //                 }
        //
        //                 if mouse_btn != MouseButton::Left {
        //                     break;
        //                 }
        //                 // Following assumption is made: We can't have up without down happening prior to it.
        //                 // The control at the mouse down position thus needs to know if the mouse was released afterwards, either inside or outside of its client area.
        //                 if let Some(control) =
        //                     window_at_point(&self.windows, self.mouse_state.lmb_down_pos)
        //                 {
        //                     self.message_queue.push((control.hwnd, Message::LmbUp));
        //                 }
        //             }
        //             Event::Window { win_event, .. } => match win_event {
        //                 WindowEvent::SizeChanged(w, h) => {
        //                     // Update this top-level window's dimensions
        //                     let top_level_window = window_from_hwnd_mut(&mut self.windows, hwnd);
        //                     top_level_window.rect = Rect {
        //                         x: 0.0,
        //                         y: 0.0,
        //                         w: w as f32 + 2.0,
        //                         h: h as f32 + 2.0,
        //                     };
        //                     self.invalidate_hwnd(hwnd);
        //                 }
        //                 _ => {}
        //             },
        //             Event::KeyDown { keycode, .. } => {
        //                 if keycode.is_none() {
        //                     break;
        //                 }
        //                 if let Some(hwnd) = self.focused_hwnd {
        //                     self.message_queue
        //                         .push((hwnd, Message::KeyDown(keycode.unwrap())));
        //                 }
        //             }
        //             Event::TextInput { text, .. } => {
        //                 self.last_text_input = text;
        //                 if let Some(hwnd) = self.focused_hwnd {
        //                     self.message_queue.push((hwnd, Message::TextInput));
        //                 }
        //             }
        //             Event::KeyUp { keycode, .. } => {
        //                 if keycode.is_none() {
        //                     break;
        //                 }
        //                 if let Some(hwnd) = self.focused_hwnd {
        //                     self.message_queue
        //                         .push((hwnd, Message::KeyUp(keycode.unwrap())));
        //                 }
        //             }
        //             _ => {}
        //         }
        //     }
    }
}
