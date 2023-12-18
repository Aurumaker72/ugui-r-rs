extern crate sdl2;

use crate::WNDPROC;
use std::collections::HashMap;

use crate::core::geo::Rect;

use crate::core::dynval;
use crate::core::styles::Styles;
use flagset::FlagSet;

use crate::HWND;

pub(crate) struct Window {
    pub hwnd: HWND,
    pub class: String,
    pub caption: String,
    pub styles: FlagSet<Styles>,
    pub rect: Rect,
    pub parent: Option<HWND>,
    pub procedure: WNDPROC,
    /// Custom data which can be retrieved and modified by user
    pub user_data: Option<Box<dyn dynval::Value>>,
}
