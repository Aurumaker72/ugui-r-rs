use ugui_r_rs::core::geo::Rect;
use ugui_r_rs::core::styles::Styles;
use ugui_r_rs::window::Ugui;
use ugui_r_rs::window::CENTER_SCREEN;

fn main() {
    let mut ugui = Ugui::default();

    let hwnd = ugui
        .create_window(
            "window".to_string(),
            "Test Window".to_string(),
            Styles::Visible | Styles::Enabled,
            Rect {
                x: CENTER_SCREEN,
                y: CENTER_SCREEN,
                w: 640.0,
                h: 480.0,
            },
            None,
        )
        .unwrap();

    ugui.show_window(hwnd);
}
