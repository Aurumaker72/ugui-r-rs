use std::ops::BitXor;
use ugui_r_rs::controls::button::{button_proc, button_style};
use ugui_r_rs::controls::textbox::{textbox_proc, textbox_style};
use ugui_r_rs::controls::window::{window_proc, window_style};
use ugui_r_rs::core::geo::Rect;
use ugui_r_rs::core::messages::Message;
use ugui_r_rs::core::styles::Styles;
use ugui_r_rs::core::ugui::Ugui;
use ugui_r_rs::CENTER_SCREEN;
use ugui_r_rs::HWND;

fn main() {
    let mut ugui = Ugui::default();

    fn my_wndproc(ugui: &mut Ugui, hwnd: HWND, message: Message) -> u64 {
        match message {
            Message::LmbDown => {
                println!("down {:?}", hwnd);
            }
            Message::User(source, kind) => match kind {
                BUTTON_CLICK => {
                    println!("Clicked {}", source);
                    if source == 1 {
                        ugui.set_window_style(source, Styles::None.into())
                    }
                    if source == 4 {
                        let style = ugui.get_window_style(source);
                        ugui.set_window_style(source, style.bitxor(Styles::Enabled))
                    }
                }
                _ => {}
            },
            _ => {}
        }
        window_proc(ugui, hwnd, message)
    }

    let hwnd = ugui
        .create_window(
            "window".to_string(),
            "Test Window".to_string(),
            window_style(),
            Rect {
                x: CENTER_SCREEN,
                y: CENTER_SCREEN,
                w: 640.0,
                h: 480.0,
            },
            None,
            my_wndproc,
        )
        .unwrap();

    let button_1_hwnd = ugui
        .create_window(
            "BUTTON".to_string(),
            "Hello World!".to_string(),
            button_style(),
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
            button_style(),
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
            button_style(),
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

    let edit_1_hwnd = ugui
        .create_window(
            "TEXTBOX".to_string(),
            "aadssd".to_string(),
            textbox_style(),
            Rect {
                x: 200.0,
                y: 50.0,
                w: 120.0,
                h: 23.0,
            },
            Some(hwnd),
            textbox_proc,
        )
        .unwrap();
    ugui.show_window(hwnd);
}
