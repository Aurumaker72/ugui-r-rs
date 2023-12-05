use ugui_r_rs::controls::control::BaseControl;
use ugui_r_rs::controls::control::Control::{Label, Stack};
use ugui_r_rs::core::geo::Alignment;
use ugui_r_rs::window::Window;
fn main() {
    let mut window = Window::new();

    window.set_content(Stack {
        base: BaseControl::new(
            Alignment::Center,
            Alignment::Center,
            vec![
                Label {
                    base: BaseControl::default(),
                    text: "testing".to_string(),
                },
                Label {
                    base: BaseControl::new(Alignment::Center, Alignment::Fill, vec![]),
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
