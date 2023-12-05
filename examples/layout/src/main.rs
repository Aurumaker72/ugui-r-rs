use ugui_r_rs::controls::control::BaseControl;
use ugui_r_rs::controls::control::Control::{Label, Stack};
use ugui_r_rs::core::geo::Alignment;
use ugui_r_rs::window::{Window, WindowBuilder};
fn main() {
    let mut window = WindowBuilder::new()
        .content(Stack {
            horizontal: true,
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
                    Stack {
                        horizontal: false,
                        base: BaseControl::new(
                            Alignment::Center,
                            Alignment::Center,
                            vec![
                                Label {
                                    base: BaseControl::new(
                                        Alignment::Start,
                                        Alignment::Center,
                                        vec![],
                                    ),
                                    text: "aaa".to_string(),
                                },
                                Label {
                                    base: BaseControl::new(
                                        Alignment::Center,
                                        Alignment::Center,
                                        vec![],
                                    ),
                                    text: "bbbb".to_string(),
                                },
                                Label {
                                    base: BaseControl::new(
                                        Alignment::End,
                                        Alignment::Center,
                                        vec![],
                                    ),
                                    text: "ccccc".to_string(),
                                },
                                Label {
                                    base: BaseControl::default(),
                                    text: "ddddddddddddddd".to_string(),
                                },
                            ],
                        ),
                    },
                    Label {
                        base: BaseControl::new(Alignment::Fill, Alignment::Center, vec![]),
                        text: "everything appears to be in order".to_string(),
                    },
                ],
            ),
        })
        .build();

    window.show();
}
