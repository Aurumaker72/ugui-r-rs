extern crate sdl2;

use crate::WNDPROC;
use std::collections::HashMap;


use crate::core::dynval;
use flagset::FlagSet;
use crate::gfx::rect::Rect;
use crate::gfx::styles::Styles;

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
