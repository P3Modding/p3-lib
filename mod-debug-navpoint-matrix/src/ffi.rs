use std::{
    mem,
    sync::atomic::{AtomicPtr, Ordering},
};

use hooklet::windows::x86::{hook_call_rel32, CallRel32Hook};
use log::debug;
use p3_api::{class35::Class35Ptr, data::ddraw_set_constant_color, mods, ui::draw_geometry_abs};

static DRAW_NAVIGATION_LINE_HOOK_PTR: AtomicPtr<CallRel32Hook> = AtomicPtr::new(std::ptr::null_mut());
const STROKE_LENGTH: i32 = 4;

#[no_mangle]
unsafe extern "C" fn start() -> u32 {
    mods::init_mod();

    debug!("Deploying draw_navigation_line hook");
    match hook_call_rel32(0x4AE38, draw_line_hook as usize as u32) {
        Ok(hook) => {
            DRAW_NAVIGATION_LINE_HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        Err(_) => return 1,
    }

    0
}

#[no_mangle]
unsafe extern "thiscall" fn draw_line_hook(this: u32, screen_rectangle: u32) {
    let class35 = Class35Ptr::new();
    ddraw_set_constant_color(0xffcc0000);
    for i in 0..class35.get_nav_vec_count() {
        let point = class35.get_nav_vec_entry(i as _);
        let x = nav_vec_coord_to_game_coord(point.0 as _);
        let y = nav_vec_coord_to_game_coord(point.1 as _);
        draw_marker(x, y);
    }

    let orig_address = (*DRAW_NAVIGATION_LINE_HOOK_PTR.load(Ordering::SeqCst)).old_absolute;
    let orig: extern "thiscall" fn(this: u32, screen_rectangle: u32) = unsafe { mem::transmute(orig_address) };
    orig(this, screen_rectangle)
}

unsafe fn draw_marker(x: i32, y: i32) {
    draw_geometry_abs(x - STROKE_LENGTH, y - STROKE_LENGTH, x + STROKE_LENGTH, y + STROKE_LENGTH);
    draw_geometry_abs(x - STROKE_LENGTH, y + STROKE_LENGTH, x + STROKE_LENGTH, y - STROKE_LENGTH);
}

fn nav_vec_coord_to_game_coord(i: i32) -> i32 {
    (26 * i + 487) * 17 / 125
}
