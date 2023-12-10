use ugui_r_rs::controls::button::button_proc;
use ugui_r_rs::core::geo::Rect;
use ugui_r_rs::core::messages::Message;
use ugui_r_rs::core::styles::Styles;
use ugui_r_rs::window::CENTER_SCREEN;
use ugui_r_rs::window::HWND;
use ugui_r_rs::window::{base_proc, Ugui};

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
            base_proc,
        )
        .unwrap();

    let button_hwnd = ugui
        .create_window(
            "BUTTON".to_string(),
            "Hello World!".to_string(),
            Styles::Visible | Styles::Enabled,
            Rect {
                x: 10.0,
                y: 10.0,
                w: 90.0,
                h: 20.0,
            },
            Some(hwnd),
            button_proc,
        )
        .unwrap();

    ugui.show_window(hwnd);
}
