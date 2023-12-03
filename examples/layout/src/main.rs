use ugui_r_rs::controls::control::Control::Label;
use ugui_r_rs::window::Window;
fn main() {
    let mut window = Window::new();

    window.set_content(Label {
        base: Default::default(),
        text: "testing".to_string(),
    });
    window.show();
}
