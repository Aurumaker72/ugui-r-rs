#[derive(Copy, Eq, PartialEq, Hash, Clone, Default, Debug)]
pub enum VisualState {
    #[default]
    Normal,
    Hover,
    Active,
    Disabled,
}
