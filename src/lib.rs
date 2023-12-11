use crate::core::messages::Message;
use crate::core::ugui::Ugui;

#[macro_use]
extern crate num_derive;
pub mod controls;
pub mod core;
pub mod window;

pub type HWND = usize;
pub type WNDPROC = fn(&mut Ugui, HWND, Message) -> u64;
