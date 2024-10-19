use std::{ffi::c_void, mem};

use screen_rectangle::Rect;

pub mod class27;
pub mod class48;
pub mod convoy;
pub mod enums;
pub mod mission;
pub mod missions;
pub mod office;
pub mod p3_ptr;
pub mod screen_game_ini_anim;
pub mod screen_rectangle;
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

/// Draw a solid rectangle at the given position.
pub fn ddraw_fill_solid_rect(x1: i32, y1: i32, width: i32, height: i32) {
    let function: extern "cdecl" fn(x1: i32, y1: i32, width: i32, height: i32) = unsafe { mem::transmute(0x004BB430) };
    function(x1, y1, width, height)
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

pub fn ddraw_copy_clipper(mode: *const c_void) {
    let function: extern "cdecl" fn(mode: *const c_void) = unsafe { mem::transmute(0x004BB280) };
    function(mode)
}

pub fn ddraw_set_active_clipper(clipper: *const c_void) -> *const c_void {
    let function: extern "cdecl" fn(clipper: *const c_void) -> *const c_void = unsafe { mem::transmute(0x004BB780) };
    function(clipper)
}

pub fn ddraw_free_rect_clipper(clipper: *const c_void) {
    let function: extern "cdecl" fn(clipper: *const c_void) = unsafe { mem::transmute(0x004BB4C0) };
    function(clipper)
}

pub fn ddraw_get_rect_clipper() -> *const c_void {
    let function: extern "cdecl" fn() -> *const c_void = unsafe { mem::transmute(0x004BB520) };
    function()
}

pub fn get_active_ddraw_clipper_index() -> u32 {
    let ptr: *const u32 = 0x006DCD04 as *const u32;
    unsafe { *ptr }
}

pub fn ddraw_copy_clipper_voodoo() {
    unsafe {
        let clippers_ptr: *const *const c_void = 0x006DCD20 as *const *const c_void;
        ddraw_copy_clipper(*clippers_ptr.add(get_active_ddraw_clipper_index() as _))
    }
}

pub fn render_window_title(text: *const c_void, window: *const c_void) {
    let function: extern "stdcall" fn(text: *const c_void, window: *const c_void) = unsafe { mem::transmute(0x00420C70) };
    function(text, window)
}

pub fn fill_p3_string(p3_string: *const c_void, input: &[u8]) {
    let function: extern "thiscall" fn(text: *const c_void, window: *const c_void) = unsafe { mem::transmute(0x0064F2C1) };
    function(p3_string, input.as_ptr() as _)
}

pub unsafe fn ui_render_text_at(x: i32, y: i32, text: &[u8]) {
    let function: extern "cdecl" fn(x: i32, y: i32, text: *const c_void) = mem::transmute(0x004BB3E0);
    function(x, y, text.as_ptr() as _)
}

pub fn class22_clip_intersection(with: *const Rect) {
    let function: extern "thiscall" fn(class22: *const c_void, rect: *const Rect) = unsafe { mem::transmute(0x0059B5B0) };
    function(get_class22_ptr(), with)
}

pub fn get_class22_ptr() -> *const c_void {
    unsafe {
        let ptr: *const *const c_void = 0x006E51AC as _;
        *ptr
    }
}
