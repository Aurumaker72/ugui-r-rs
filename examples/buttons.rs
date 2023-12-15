use std::ops::BitXor;
use ugui_r_rs::controls::button::{button_proc, button_style};
use ugui_r_rs::controls::scrollbar::{scrollbar_proc, scrollbar_style};
use ugui_r_rs::controls::textbox::textbox_proc;
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
                _BUTTON_CLICK => {
                    ugui.set_caption(hwnd, "a".to_string());
                    // ugui.destroy_window(source);
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

    for i in 0..5 {
        for j in 0..5 {
            ugui.create_window(
                "BUTTON".to_string(),
                "Hello World!".to_string(),
                button_style(),
                Rect {
                    x: (i as f32 * 90.0) + 10.0 + (i as f32 * 2.0),
                    y: (j as f32 * 20.0) + 10.0 + (j as f32 * 2.0),
                    w: 90.0,
                    h: 20.0,
                },
                Some(hwnd),
                button_proc,
            )
            .unwrap();
        }
    }

    for i in 0..5 {
        for j in 0..5 {
            ugui.create_window(
                "EDIT".to_string(),
                "Hello World!".to_string(),
                button_style(),
                Rect {
                    x: (i as f32 * 90.0) + 10.0 + (i as f32 * 2.0),
                    y: (j as f32 * 20.0) + 135.0 + (j as f32 * 2.0),
                    w: 90.0,
                    h: 20.0,
                },
                Some(hwnd),
                textbox_proc,
            )
            .unwrap();
        }
    }

    for i in 0..20 {
        for j in 0..2 {
            ugui.create_window(
                "SCROLL".to_string(),
                Default::default(),
                scrollbar_style(),
                Rect {
                    x: (i as f32 * 20.0) + 10.0 + (i as f32 * 2.0),
                    y: (j as f32 * 120.0) + 250.0 + (j as f32 * 2.0),
                    w: 20.0,
                    h: 120.0,
                },
                Some(hwnd),
                scrollbar_proc,
            )
            .unwrap();
        }
    }

    ugui.show_window(hwnd);
}
