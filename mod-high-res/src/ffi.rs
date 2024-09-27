use std::{
    arch::global_asm,
    ffi::{c_void, CStr},
    mem, ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

use hooklet::{CallRel32Hook, X86Rel32Type};
use log::{debug, warn};
use p3_api::{
    data::{class27::Class27Ptr, ddraw_fill_solid_rect, ddraw_set_constant_color, ddraw_set_render_dest, get_resolution_height, get_resolution_width},
    ui::class73::Class73Ptr,
};
use windows::core::PCSTR;

const FULLHD_STRING: &CStr = c"1920 x 1080";

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
static DECODE_SUPPORTED_FILES_HOOK_PTR: AtomicPtr<CallRel32Hook> = AtomicPtr::new(std::ptr::null_mut());
// Overwrite accelMap resolution
static LOAD_ACCEL_MAP_INI_HOOK_PTR: AtomicPtr<CallRel32Hook> = AtomicPtr::new(std::ptr::null_mut());

static VOLLANSICHTSKARTE1920: &[u8] = include_bytes!("Vollansichtskarte1920.bmp");

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
        return 1;
    }

    // Replace the mapping from class24 resolution field to width and height values
    if hooklet::deploy_rel32_raw(
        CALCULATE_RESOLUTION_BEFORE_SCENE_LOAD_PATCH_ADDRESS as _,
        (&calculate_resolution_before_scene_load) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 2;
    }

    // Replace the mapping from ui options resolution field to width and height values
    if hooklet::deploy_rel32_raw(
        CALCULATE_RESOLUTION_AFTER_SCENE_LOAD_PATCH_ADDRESS as _,
        (&calculate_resolution_after_scene_load) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 3;
    }

    debug!("Deploying resolution dependent ui positioning patch");
    if hooklet::deploy_rel32_raw(
        REPOSITION_UI_ELEMENTS_PATCH_ADDRESS as _,
        (&reposition_ui_elements) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 4;
    }

    debug!("Deploying resolution dependent top bar ui positioning patch");
    if hooklet::deploy_rel32_raw(
        REPOSITION_UI_ELEMENTS_TOP_BAR_PATCH_ADDRESS as _,
        (&reposition_ui_elements_top_bar) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 5;
    }

    // Fix empty bottom right corner
    debug!("Deploying render_all_objects_hook");
    match hooklet::hook_call_rel32(PCSTR::from_raw(ptr::null()), 0x28649, maybe_render_all_objects_hook as usize) {
        Ok(hook) => {
            HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        Err(_) => return 6,
    }

    // Fix acceleration map
    debug!("Deploying dddraw_dll.dll decode_supported_files hook to replace the background image");
    match hooklet::hook_call_rel32(
        PCSTR::from_raw(c"aim.dll".as_ptr() as _),
        0x2984,
        ddraw_dll_decode_supported_files_hook as usize,
    ) {
        Ok(hook) => {
            debug!("Hook {hook:?} set");
            DECODE_SUPPORTED_FILES_HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        Err(_) => return 7,
    }

    // Fix acceleration map
    debug!("Deploying load screen settings from accelMap.ini");
    match hooklet::hook_call_rel32(PCSTR::from_raw(ptr::null()), 0x12C5, class73_place_ui_element_hook as usize) {
        Ok(hook) => {
            debug!("Hook {hook:?} set");
            LOAD_ACCEL_MAP_INI_HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        Err(_) => return 8,
    }

    let resolution_pcstr_ptr: *mut *const i8 = 0x0069AE40 as _;
    *resolution_pcstr_ptr = FULLHD_STRING.as_ptr();

    debug!("Mod loaded successfully");
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
    let class27 = Class27Ptr::new();
    let anim42 = class27.get_anim_42();
    let rect = anim42.get_screen_rectangle(0);
    let new_width = real_resolution_width - 284;
    debug!("{rect:X?} setting new ANIM42 width: {new_width}");
    rect.set_width(new_width);

    // Fix ANIM44 Pos
    let anim42 = class27.get_anim_44();
    anim42.set_pos_x(real_resolution_width as i32 - 284);

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
    let res_width = get_resolution_width() as i32;
    let res_height = get_resolution_height() as i32;

    let old_render_dest = ddraw_set_render_dest(-1);
    ddraw_set_constant_color(0xff000000);
    ddraw_fill_solid_rect(res_width - 284, 1024, res_width, res_height);
    ddraw_set_render_dest(old_render_dest);

    0
}

#[no_mangle]
pub unsafe extern "cdecl" fn ddraw_dll_decode_supported_files_hook(aim_image_inner: u32, file_path: PCSTR, mut file_data: u32, mut file_size: u32) -> i32 {
    let orig_address = (*DECODE_SUPPORTED_FILES_HOOK_PTR.load(Ordering::SeqCst)).old_absolute;

    match file_path.to_string() {
        Ok(path) => {
            if path.ends_with("Vollansichtskarte1280.bmp") {
                debug!("ddraw_dll_decode_supported_files_hook replacing Vollansichtskarte1280");
                file_data = VOLLANSICHTSKARTE1920.as_ptr() as _;
                file_size = VOLLANSICHTSKARTE1920.len() as _;
            }
        }
        Err(e) => warn!("{e:?}"), // These are probably not utf8 but ansi?
    }

    let orig: extern "cdecl" fn(aim_image_inner: u32, file_path: PCSTR, file_data: u32, file_size: u32) -> i32 = unsafe { mem::transmute(orig_address) };
    orig(aim_image_inner, file_path, file_data, file_size)
}

#[no_mangle]
pub unsafe extern "thiscall" fn class73_place_ui_element_hook(this: u32, zero: u32, class74: u32) {
    let class73 = Class73Ptr::new();
    class73.set_x(0);
    class73.set_y(0);
    class73.set_width(1920);
    class73.set_height(1080);
    class73.get_anim_0_1_2().get_screen_rectangle(0).set_width(1920);
    class73.get_anim_0_1_2().get_screen_rectangle(0).set_height(1080);

    let orig_address = (*LOAD_ACCEL_MAP_INI_HOOK_PTR.load(Ordering::SeqCst)).old_absolute;
    let orig: extern "thiscall" fn(this: u32, zero: u32, class74: u32) = unsafe { mem::transmute(orig_address) };
    debug!("class73_place_ui_element_hook calling {orig_address:#X}");
    orig(this, zero, class74)
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
