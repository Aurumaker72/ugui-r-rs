#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Default for Color {
    fn default() -> Self {
        Color::RED
    }
}
impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn hex(str: &str) -> Color {
        let r = &str[1..3];
        let g = &str[3..5];
        let b = &str[5..7];
        Color::rgb(
            u8::from_str_radix(r, 16).unwrap(),
            u8::from_str_radix(g, 16).unwrap(),
            u8::from_str_radix(b, 16).unwrap(),
        )
    }
}
