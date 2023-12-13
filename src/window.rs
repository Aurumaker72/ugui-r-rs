extern crate sdl2;

use crate::WNDPROC;

use crate::core::geo::{Rect};

use crate::core::styles::Styles;
use flagset::FlagSet;








use crate::HWND;

#[derive(Clone)]
pub(crate) struct Window {
    pub hwnd: HWND,
    pub class: String,
    pub caption: String,
    pub styles: FlagSet<Styles>,
    pub rect: Rect,
    pub parent: Option<HWND>,
    pub procedure: WNDPROC,
    pub state_0: u64,
}