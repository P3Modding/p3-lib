use std::{
    arch::global_asm,
    ffi::c_void,
    mem, ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

use hooklet::{CallRel32Hook, X86Rel32Type};
use log::debug;
use p3_api::{
    data::{class27::Class27Ptr, ddraw_fill_solid_rect, ddraw_set_constant_color, ddraw_set_render_dest, get_resolution_height, get_resolution_width},
    p3_access_api::raw_p3_access_api::RawP3AccessApi,
};
use windows::core::PCSTR;

pub const P3: RawP3AccessApi = RawP3AccessApi::new();

const CALCULATE_RESOLUTION_ON_OPTIONS_MENU_CLOSE_PATCH_ADDRESS: u32 = 0x00423BBD;
static CALCULATE_RESOLUTION_ON_OPTIONS_MENU_CLOSE_CONTINUATION: u32 = 0x00423C00;

// Overwrite calculated results from class24 resolution field
const CALCULATE_RESOLUTION_BEFORE_SCENE_LOAD_PATCH_ADDRESS: u32 = 0x00432D1E;
static CALCULATE_RESOLUTION_BEFORE_SCENE_LOAD_CONTINUATION: u32 = 0x00432D58;

// Overwrite calculated results ui options field
const CALCULATE_RESOLUTION_AFTER_SCENE_LOAD_PATCH_ADDRESS: u32 = 0x00463FBB;
static CALCULATE_RESOLUTION_AFTER_SCENE_LOAD_CONTINUATION: u32 = 0x00463FFD;

// Overwrite switch value and 1280x1024 UI values, if required
const REPOSITION_UI_ELEMENTS_PATCH_ADDRESS: u32 = 0x00429AC0; // Detour instead of loading width
static REPOSITION_UI_ELEMENTS_CONTINUATION_ADDRESS: u32 = 0x00429AC5; // Jump back to switch

// Overwrite top bar positioning switch resolution value
const REPOSITION_UI_ELEMENTS_TOP_BAR_PATCH_ADDRESS: u32 = 0x00429FE0; // Detour instead of loading width
static REPOSITION_UI_ELEMENTS_TOP_BAR_CONTINUATION_ADDRESS: u32 = 0x00429FE5; // Jump back to switch

static HOOK_PTR: AtomicPtr<CallRel32Hook> = AtomicPtr::new(std::ptr::null_mut());

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    if hooklet::deploy_rel32_raw(
        CALCULATE_RESOLUTION_ON_OPTIONS_MENU_CLOSE_PATCH_ADDRESS as _,
        (&calculate_resolution_after_options_screen) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 0;
    }

    // Replace the mapping from class24 resolution field to width and height values
    if hooklet::deploy_rel32_raw(
        CALCULATE_RESOLUTION_BEFORE_SCENE_LOAD_PATCH_ADDRESS as _,
        (&calculate_resolution_before_scene_load) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 1;
    }

    // Replace the mapping from ui options resolution field to width and height values
    if hooklet::deploy_rel32_raw(
        CALCULATE_RESOLUTION_AFTER_SCENE_LOAD_PATCH_ADDRESS as _,
        (&calculate_resolution_after_scene_load) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 2;
    }

    debug!("Deploying resolution dependent ui positioning patch");
    if hooklet::deploy_rel32_raw(
        REPOSITION_UI_ELEMENTS_PATCH_ADDRESS as _,
        (&reposition_ui_elements) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 3;
    }

    debug!("Deploying resolution dependent top bar ui positioning patch");
    if hooklet::deploy_rel32_raw(
        REPOSITION_UI_ELEMENTS_TOP_BAR_PATCH_ADDRESS as _,
        (&reposition_ui_elements_top_bar) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 4;
    }

    match hooklet::hook_call_rel32(PCSTR::from_raw(ptr::null()), 0x28649, maybe_render_all_objects_hook as usize) {
        Ok(hook) => {
            HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        Err(_) => return 5,
    }

    0
}

#[no_mangle]
pub unsafe extern "cdecl" fn overwrite_resolution(width: *mut u32, height: *mut u32) {
    debug!("overwrite_resolution() patching width and height");
    *width = 1920;
    *height = 1080;
}

/// Overwrite the ANIM* entries from  screenGame.ini, if a custom resolution is selected.
///
/// Returns the real vanilla resolution width, or 1280.
#[no_mangle]
pub unsafe extern "cdecl" fn reposition_ui_elements_custom() -> u32 {
    debug!("reposition_ui_elements_custom()");
    let real_resolution_width = get_resolution_width();

    if real_resolution_width <= 1280 {
        debug!("Returning real resolution width {}", real_resolution_width);
        return real_resolution_width;
    }

    // Fix ANIM42 Frame0 width
    let class27 = Class27Ptr::<RawP3AccessApi>::new(&P3).unwrap();
    let anim42 = class27.get_anim_42();
    let rect = anim42.get_screen_rectangle(0, &P3).unwrap();
    let new_width = real_resolution_width - 284;
    debug!("{rect:X?} setting new ANIM42 width: {new_width}");
    rect.set_width(new_width, &P3).unwrap();

    // Fix ANIM44 Pos
    let anim42 = class27.get_anim_44();
    anim42.set_pos_x(real_resolution_width - 284, &P3).unwrap();

    1280
}

#[no_mangle]
pub unsafe extern "cdecl" fn reposition_ui_elements_top_bar_custom() -> u32 {
    debug!("reposition_ui_elements_top_bar_custom()");
    let real_resolution_width = get_resolution_width();

    if real_resolution_width <= 1280 {
        debug!("Returning real resolution width {}", real_resolution_width);
        real_resolution_width
    } else {
        debug!("Returning 1280 resolution width instead of {}", real_resolution_width);
        1280
    }
}

#[no_mangle]
pub unsafe extern "thiscall" fn maybe_render_all_objects_hook(this: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> i32 {
    let orig: extern "thiscall" fn(this: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> i32 = unsafe { mem::transmute(0x004B58C0) };
    orig(this, a2, a3, a4, a5);
    let res_width = get_resolution_width();
    let res_height = get_resolution_height();

    let old_render_dest = ddraw_set_render_dest(-1);
    ddraw_set_constant_color(0xff000000);
    ddraw_fill_solid_rect(res_width - 284, 1024, res_width, res_height);
    ddraw_set_render_dest(old_render_dest);

    0
}

extern "C" {
    static calculate_resolution_after_options_screen: c_void;
    static calculate_resolution_before_scene_load: c_void;
    static calculate_resolution_after_scene_load: c_void;
    static reposition_ui_elements: c_void;
    static reposition_ui_elements_top_bar: c_void;
}

global_asm!("
.global {calculate_resolution_after_options_screen}
{calculate_resolution_after_options_screen}:
# save regs
push eax
push ecx
push edx

lea ecx, [esp + 0x4C + 0x0C]
lea eax, [esp + 0x50 + 0x0C]
push eax
push ecx
call {overwrite_resolution}
pop eax
pop eax

# restore regs
pop edx
pop ecx
pop eax

jmp [{continuation}]
",
calculate_resolution_after_options_screen = sym calculate_resolution_after_options_screen,
overwrite_resolution = sym overwrite_resolution,
continuation = sym CALCULATE_RESOLUTION_ON_OPTIONS_MENU_CLOSE_CONTINUATION);

global_asm!("
.global {calculate_resolution_before_scene_load}
{calculate_resolution_before_scene_load}:
# save regs
push eax
push ecx
push edx

lea ecx, [esp + 0x3C + 0x0C]
lea eax, [esp + 0x40 + 0x0C]
push eax
push ecx
call {overwrite_resolution}
#TODO sub
pop eax
pop eax

# restore regs
pop edx
pop ecx
pop eax

jmp [{continuation}]
",
calculate_resolution_before_scene_load = sym calculate_resolution_before_scene_load,
overwrite_resolution = sym overwrite_resolution,
continuation = sym CALCULATE_RESOLUTION_BEFORE_SCENE_LOAD_CONTINUATION);

global_asm!("
.global {calculate_resolution_after_scene_load}
{calculate_resolution_after_scene_load}:
# save regs
push eax
push ecx
push edx

lea ecx, [esp + 0x24 + 0x0C]
lea eax, [esp + 0x28 + 0x0C]
push eax
push ecx
call {overwrite_resolution}
pop eax
pop eax

# restore regs
pop edx
pop ecx
pop eax

jmp [{continuation}]
",
calculate_resolution_after_scene_load = sym calculate_resolution_after_scene_load,
overwrite_resolution = sym overwrite_resolution,
continuation = sym CALCULATE_RESOLUTION_AFTER_SCENE_LOAD_CONTINUATION);

global_asm!("
.global {reposition_ui_elements}
{reposition_ui_elements}:
# save regs
push ecx
push edx

# set eax to shim width, and manipulate the corresponding coordiantes
call {reposition_ui_elements_custom}

# restore regs
pop edx
pop ecx

# jump to switch
jmp    [{continuation_address}]
",
reposition_ui_elements = sym reposition_ui_elements,
reposition_ui_elements_custom = sym reposition_ui_elements_custom,
continuation_address = sym REPOSITION_UI_ELEMENTS_CONTINUATION_ADDRESS);

global_asm!("
.global {reposition_ui_elements_top_bar}
{reposition_ui_elements_top_bar}:
# save regs
push ecx
push edx

call {reposition_ui_elements_top_bar_custom}

# restore regs
pop edx
pop ecx

# jump to switch
jmp [{continuation_address}",
reposition_ui_elements_top_bar = sym reposition_ui_elements_top_bar,
reposition_ui_elements_top_bar_custom = sym reposition_ui_elements_top_bar_custom,
continuation_address = sym REPOSITION_UI_ELEMENTS_TOP_BAR_CONTINUATION_ADDRESS);
