use crate::core::geo::{Point, Rect};
use crate::core::styles::Styles;
use flagset::FlagSet;

/// A basic context used by the Paint message to draw a control
pub struct PaintContext {
    pub size: Point,
    pub color: Box<dyn FnMut(u8, u8, u8)>,
    pub rect: Box<dyn FnMut(Rect)>,
}

/// Messages which are sent to controls when application events happen
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
    /// Contains the mouse position relative to the control
    MouseMove(Point),

    /// The control gained focus
    Focus,

    /// The control lost focus
    Unfocus,

    /// The control needs repainting
    /// Contains a context to draw with
    Paint(PaintContext),

    /// The control's styles have changed
    /// Contains the control's new styles
    StylesChanged(FlagSet<Styles>),
}
