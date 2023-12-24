#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub enum Alignment {
    #[default]
    Start,
    Center,
    End,
    Fill,
}
