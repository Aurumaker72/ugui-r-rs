#[derive(Eq, PartialEq, Hash, Clone)]
pub enum VisualState {
    Normal,
    Hover,
    Active,
    Disabled,
}
