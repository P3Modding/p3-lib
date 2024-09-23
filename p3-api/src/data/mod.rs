use std::{ffi::c_void, mem};

pub mod class27;
pub mod class48;
pub mod convoy;
pub mod enums;
pub mod mission;
pub mod missions;
pub mod office;
pub mod operation;
pub mod p3_ptr;
pub mod screen_game_ini_anim;
pub mod screen_rectangle;
pub mod ship;
pub mod ships;
pub mod statics;
pub mod storage;

const RESOLUTION_WIDTH_PTR: *const u32 = 0x006DCCE0 as _;
const RESOLUTION_HEIGHT_PTR: *const u32 = 0x006DCCE4 as _;

//TODO refactor those behind P3 api object to allow outside use
pub fn get_resolution_width() -> u32 {
    unsafe { *RESOLUTION_WIDTH_PTR }
}

pub fn get_resolution_height() -> u32 {
    unsafe { *RESOLUTION_HEIGHT_PTR }
}

pub fn ddraw_set_constant_color(color: u32) {
    let function: extern "cdecl" fn(color: u32) = unsafe { mem::transmute(0x0500F790) };
    function(color)
}

pub fn ddraw_fill_solid_rect(x1: u32, y1: u32, x2: u32, y2: u32) {
    let function: extern "cdecl" fn(x1: u32, y1: u32, x2: u32, y2: u32) = unsafe { mem::transmute(0x004BB430) };
    function(x1, y1, x2, y2)
}

pub fn ddraw_set_render_dest(dest: i32) -> i32 {
    let function: extern "cdecl" fn(dest: i32) -> i32 = unsafe { mem::transmute(0x004BB9B0) };
    function(dest)
}

pub fn ddraw_set_2d_clipping_mode(mode: u32) -> u32 {
    let function: extern "cdecl" fn(dest: u32) -> u32 = unsafe { mem::transmute(0x004BB760) };
    function(mode)
}

pub fn ddraw_set_clip_rect(x: u32, y: u32, width: u32, height: u32) {
    let function: extern "cdecl" fn(x: u32, y: u32, width: u32, height: u32) = unsafe { mem::transmute(0x004BB800) };
    function(x, y, width, height)
}

pub fn ddraw_add_clip_rect(rect: [u32; 4]) {
    let function: extern "cdecl" fn(rect: *const u32) = unsafe { mem::transmute(0x004BB140) };
    function(rect.as_ptr())
}

pub fn ddraw_set_clip_rect_r(rect: [u32; 4]) {
    let function: extern "cdecl" fn(rect: *const u32) = unsafe { mem::transmute(0x004BB820) };
    function(rect.as_ptr())
}

pub fn ddraw_set_text_mode(mode: u32) {
    let function: extern "cdecl" fn(mode: u32) = unsafe { mem::transmute(0x004BBA10) };
    function(mode)
}

pub fn render_window_title(text: *const c_void, window: *const c_void) {
    let function: extern "stdcall" fn(text: *const c_void, window: *const c_void) = unsafe { mem::transmute(0x00420C70) };
    function(text, window)
}

pub fn fill_p3_string(p3_string: *const c_void, input: &[u8]) {
    let function: extern "thiscall" fn(text: *const c_void, window: *const c_void) = unsafe { mem::transmute(0x0064F2C1) };
    function(p3_string, input.as_ptr() as _)
}

pub fn ui_render_text_at(x: i32, y: i32, text: &[u8]) {
    let function: extern "cdecl" fn(x: i32, y: i32, text: *const c_void) = unsafe { mem::transmute(0x004BB3E0) };
    function(x, y, text.as_ptr() as _)
}
