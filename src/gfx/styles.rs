use flagset::flags;

flags! {
    pub enum Styles: u64 {
        None,
        Visible,
        Enabled,
        Focusable,
    }
}