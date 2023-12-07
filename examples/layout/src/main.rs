use ugui_r_rs::core::geo::Rect;
use ugui_r_rs::core::messages::Message;
use ugui_r_rs::core::styles::Styles;
use ugui_r_rs::window::Ugui;
use ugui_r_rs::window::CENTER_SCREEN;
use ugui_r_rs::window::HWND;

fn main() {
    let mut ugui = Ugui::default();

    fn wndproc(hwnd: HWND, message: Message) -> u64 {
        match message {
            Message::Create => {
                println!("Hello");
            }
            Message::Destroy => {
                println!("Goodbye");
            }
        }
        return 0
    }

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
            Some(wndproc),
        )
        .unwrap();

    ugui.show_window(hwnd);
}
