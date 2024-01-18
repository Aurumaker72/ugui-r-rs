use crate::gfx::alignment::Alignment;
use crate::gfx::color::Color;
use crate::gfx::rect::Rect;
use ggez::graphics;
use ggez::graphics::Canvas;

pub trait Painter {
    /// Paints a decorated quad
    ///
    /// # Arguments
    ///
    /// * `rect`: The quad destination
    /// * `back_color`: The quad's background color
    /// * `border_color`: The quad's border color
    /// * `border_size`: The quad's border size
    fn paint_quad(&mut self, rect: Rect, back_color: Color, border_color: Color, border_size: f32);

    /// Paints text
    ///
    /// # Arguments
    ///
    /// * `text`: The text
    /// * `rect`: The text bounds
    /// * `color`: The text color
    /// * `horizontal_alignment`: The text's horizontal alignment inside its bounds
    /// * `vertical_alignment`: The text's vertical alignment inside its bounds
    /// * `size`: The size
    /// * `line_height`: The space between line breaks
    fn paint_text<'a>(
        &mut self,
        text: &str,
        rect: Rect,
        color: Color,
        horizontal_alignment: Alignment,
        vertical_alignment: Alignment,
        size: f32,
        line_height: f32,
    );

    /// Pushes a clipping region to the clip stack
    ///
    /// # Arguments
    ///
    /// * `rect`: The clipping region
    fn push_clip(&mut self, rect: Rect);


    /// Pops the topmost clipping region off the clip stack
    fn pop_clip(&mut self);
}