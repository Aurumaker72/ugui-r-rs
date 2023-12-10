#[derive(Eq, PartialEq, Hash, FromPrimitive, ToPrimitive)]
pub enum VisualState {
    Normal,
    Hover,
    Active,
    Disabled,
}
