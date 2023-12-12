use flagset::flags;
use sdl2::pixels::Color;

flags! {
    pub enum Styles: u64 {
        None,
        Visible,
        Enabled,
    }
}

pub fn hex_color(str: &str) -> Color {
    let r = &str[1..3];
    let g = &str[3..5];
    let b = &str[5..7];
    Color::RGB(
        u8::from_str_radix(r, 16).unwrap(),
        u8::from_str_radix(g, 16).unwrap(),
        u8::from_str_radix(b, 16).unwrap(),
    )
}