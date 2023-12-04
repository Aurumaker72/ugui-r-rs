use std::rc::Rc;
use ugui_r_rs::controls::control::BaseControl;
use ugui_r_rs::controls::control::Control::{Label, Stack};
use ugui_r_rs::window::Window;
fn main() {
    let mut window = Window::new();

    window.set_content(Stack {
        base: BaseControl::new(
            Default::default(),
            Default::default(),
            Default::default(),
            vec![
                Label {
                    base: BaseControl::default(),
                    text: "testing".to_string(),
                },
                Label {
                    base: BaseControl::default(),
                    text: "testing".to_string(),
                },
                Label {
                    base: BaseControl::default(),
                    text: "testing".to_string(),
                },
                Label {
                    base: BaseControl::default(),
                    text: "everything appears to be in order".to_string(),
                },
            ],
        ),
    });
    window.show();
}
