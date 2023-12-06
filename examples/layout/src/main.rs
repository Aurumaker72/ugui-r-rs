use ugui_r_rs::controls::control::Control::{Label, Stack};
use ugui_r_rs::controls::control::{BaseControl, Orientation};
use ugui_r_rs::core::geo::Alignment;
use ugui_r_rs::core::messages::Message;
use ugui_r_rs::window::{Window, WindowBuilder};

fn main() {
    let mut window = WindowBuilder::new()
        .content(Stack {
            orientation: Orientation::Horizontal,
            base: BaseControl {
                on_message: |msg| println!("{:?}", msg),
                h_align: Alignment::Center,
                v_align: Alignment::Center,
                children: vec![
                    Stack {
                        orientation: Orientation::Vertical,
                        base: BaseControl {
                            h_align: Alignment::Fill,
                            v_align: Alignment::Fill,
                            children: vec![
                                Label {
                                    base: BaseControl {
                                        h_align: Alignment::Start,
                                        ..Default::default()
                                    },
                                    text: "Start".to_string(),
                                },
                                Label {
                                    base: BaseControl {
                                        h_align: Alignment::Center,
                                        ..Default::default()
                                    },
                                    text: "Center".to_string(),
                                },
                                Label {
                                    base: BaseControl {
                                        h_align: Alignment::End,
                                        ..Default::default()
                                    },
                                    text: "End".to_string(),
                                },
                                Label {
                                    base: BaseControl {
                                        h_align: Alignment::Fill,
                                        ..Default::default()
                                    },
                                    text: "Fill".to_string(),
                                },
                            ],
                            ..Default::default()
                        },
                    },
                    Stack {
                        orientation: Orientation::Horizontal,
                        base: BaseControl {
                            h_align: Alignment::Fill,
                            v_align: Alignment::Fill,
                            children: vec![
                                Label {
                                    base: BaseControl {
                                        v_align: Alignment::Start,
                                        ..Default::default()
                                    },
                                    text: "Start".to_string(),
                                },
                                Label {
                                    base: BaseControl {
                                        v_align: Alignment::Center,
                                        ..Default::default()
                                    },
                                    text: "Center".to_string(),
                                },
                                Label {
                                    base: BaseControl {
                                        v_align: Alignment::End,
                                        ..Default::default()
                                    },
                                    text: "End".to_string(),
                                },
                                Label {
                                    base: BaseControl {
                                        v_align: Alignment::Fill,
                                        ..Default::default()
                                    },
                                    text: "Fill".to_string(),
                                },
                            ],
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
        })
        .build();

    window.show();
}
