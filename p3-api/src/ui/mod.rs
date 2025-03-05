use std::mem;

use class37::Class37Ptr;

use crate::data::{screen_rectangle::Rect, ui_render_text_at};

pub mod class37;
pub mod class73;
pub mod ddraw;
pub mod font;
pub mod ui_shipyard_window;
pub mod ui_town_hall_sidemenu;
pub mod ui_town_hall_window;
pub mod ui_trading_office_window;

pub unsafe fn rect_clipper_stuff(rect: *const Rect) {
    let function: extern "stdcall" fn(rect: *const Rect) = mem::transmute(0x004B9650);
    function(rect)
}

pub fn draw_geometry(x1: i32, y1: i32, x2: i32, y2: i32) {
    let function: extern "cdecl" fn(x1: i32, y1: i32, x2: i32, y2: i32) = unsafe { mem::transmute(0x004BD680) };
    function(x1, y1, x2, y2)
}

pub unsafe fn draw_geometry_abs(x1: i32, y1: i32, x2: i32, y2: i32) {
    let class37 = Class37Ptr::new();
    let function: extern "cdecl" fn(x1: i32, y1: i32, x2: i32, y2: i32) = unsafe { mem::transmute(0x004BD680) };
    function(
        x1 - class37.get_offset_x() as i32 + class37.get_x(),
        y1 - class37.get_offset_y() as i32 + class37.get_y(),
        x2 - class37.get_offset_x() as i32 + class37.get_x(),
        y2 - class37.get_offset_y() as i32 + class37.get_y(),
    )
}

pub unsafe fn draw_text_at_abs(x: i32, y: i32, text: &[u8]) {
    let class37 = Class37Ptr::new();
    ui_render_text_at(
        x - class37.get_offset_x() as i32 + class37.get_x(),
        y - class37.get_offset_y() as i32 + class37.get_y(),
        text,
    )
}
