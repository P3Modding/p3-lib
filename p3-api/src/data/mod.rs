use std::mem;

pub mod class27;
pub mod convoy;
pub mod enums;
pub mod game_world;
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
pub mod town;

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
