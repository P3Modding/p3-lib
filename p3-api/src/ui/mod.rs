use std::mem;

use crate::data::screen_rectangle::Rect;

pub mod class37;
pub mod class73;
pub mod font;
pub mod ui_shipyard_window;

pub unsafe fn rect_clipper_stuff(rect: *const Rect) {
    let function: extern "stdcall" fn(rect: *const Rect) = mem::transmute(0x004B9650);
    function(rect)
}
