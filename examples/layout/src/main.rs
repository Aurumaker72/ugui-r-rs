use std::rc::Rc;
use ugui_r_rs::controls::control::BaseControl;
use ugui_r_rs::controls::control::Control::{Label, Stack};
use ugui_r_rs::window::Window;
fn main() {
    let mut window = Window::new();

    window.set_content(Stack {
        base: BaseControl {
            children: vec![
                Rc::new(Label {
                    base: BaseControl {
                        ..Default::default()
                    },
                    text: "testing".to_string(),
                }),
                Rc::new(Label {
                    base: BaseControl {
                        ..Default::default()
                    },
                    text: "testing".to_string(),
                }),
                Rc::new(Label {
                    base: BaseControl {
                        ..Default::default()
                    },
                    text: "testing".to_string(),
                }),
                Rc::new(Label {
                    base: BaseControl {
                        ..Default::default()
                    },
                    text: "everything appears to be in order".to_string(),
                }),
            ],
            ..Default::default()
        },
    });
    window.show();
}
