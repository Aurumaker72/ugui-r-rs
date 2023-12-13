use sdl2::keyboard::Keycode;
use crate::HWND;

/// Messages which are sent to controls when application events happen
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Message {
    /// The control has been created
    Create,

    /// The control is being destroyed
    Destroy,

    /// The primary mouse button is pushed down while inside the control
    LmbDown,

    /// The primary mouse button is released with focus inside the control
    LmbUp,

    /// The mouse is moving while inside the control
    MouseMove,

    /// The mouse has entered the control bounds
    MouseEnter,

    /// The mouse has left the control bounds
    MouseLeave,

    /// A keyboard key was pushed with focus inside the control
    KeyDown(Keycode),

    /// A keyboard key was released with focus inside the control
    KeyUp(Keycode),

    /// Text was written by the user
    TextInput,

    /// The control gained focus
    Focus,

    /// The control lost focus
    Unfocus,

    /// The control needs repainting
    Paint,

    /// The control's styles have changed
    StylesChanged,

    /// A user-defined message
    /// Contains the source window's handle (e.g.: a button), and a unique message identifier
    User(HWND, u64)
}
