use ugui_r_rs::window::base_proc;
use ugui_r_rs::core::ugui::Ugui;
use ugui_r_rs::controls::button::button_proc;
use ugui_r_rs::core::geo::Rect;
use ugui_r_rs::core::messages::Message;
use ugui_r_rs::core::styles::Styles;
use ugui_r_rs::CENTER_SCREEN;
use ugui_r_rs::HWND;

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

    let button_1_hwnd = ugui
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

    let button_2_hwnd = ugui
        .create_window(
            "BUTTON".to_string(),
            "im a disabled button".to_string(),
            Styles::Visible.into(),
            Rect {
                x: 10.0,
                y: 40.0,
                w: 90.0,
                h: 20.0,
            },
            Some(hwnd),
            button_proc,
        )
        .unwrap();

    let button_3_hwnd = ugui
        .create_window(
            "BUTTON".to_string(),
            "im an invisible button".to_string(),
            Styles::None.into(),
            Rect {
                x: 10.0,
                y: 70.0,
                w: 90.0,
                h: 20.0,
            },
            Some(hwnd),
            button_proc,
        )
        .unwrap();

    let button_4_hwnd = ugui
        .create_window(
            "BUTTON".to_string(),
            "Hello World asdsadads!".to_string(),
            Styles::Visible | Styles::Enabled,
            Rect {
                x: 120.0,
                y: 10.0,
                w: 120.0,
                h: 20.0,
            },
            Some(hwnd),
            button_proc,
        )
        .unwrap();

    let button_5_hwnd = ugui
        .create_window(
            "BUTTON".to_string(),
            "we overlap".to_string(),
            Styles::Visible | Styles::Enabled,
            Rect {
                x: 130.0,
                y: 20.0,
                w: 120.0,
                h: 20.0,
            },
            Some(hwnd),
            button_proc,
        )
        .unwrap();
    ugui.show_window(hwnd);
}
