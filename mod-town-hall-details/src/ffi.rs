use std::{
    arch::global_asm,
    ffi::c_void,
    mem, ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

use hooklet::windows::x86::{deploy_rel32_raw, CallRel32Hook, FunctionPointerHook, X86Rel32Type};
use hooklet::windows::x86::{hook_call_rel32, hook_function_pointer_width_module};
use log::debug;
use p3_api::ui::ui_town_hall_window::UITownHallWindowPtr;
use windows::core::PCSTR;

const TOWN_HALL_WINDOW_OPEN_POINTER_OFFSET: u32 = UITownHallWindowPtr::VTABLE_OFFSET + 0x120;
pub static TOWN_HALL_WINDOW_OPEN_HOOK_PTR: AtomicPtr<FunctionPointerHook> = AtomicPtr::new(std::ptr::null_mut());

const TOWN_HALL_SIDEPANEL_SET_SELECTED_PAGE_PATCH_OFFSET: u32 = 0x1A94BC;
pub static TOWN_HALL_SIDEPANEL_SET_SELECTED_PAGE_HOOK_PTR: AtomicPtr<CallRel32Hook> = AtomicPtr::new(std::ptr::null_mut());

const LOAD_TOWN_HALL_SELECTED_PAGE_PATCH_ADDRESS: u32 = 0x005E09AC;
static LOAD_TOWN_HALL_SELECTED_PAGE_CONTINUATION: u32 = 0x005E09B2;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    debug!("Hooking town hall window's open function through vtable");
    match hook_function_pointer_width_module(
        PCSTR::from_raw(ptr::null()),
        TOWN_HALL_WINDOW_OPEN_POINTER_OFFSET,
        town_hall_window_open_hook as usize as u32,
    ) {
        Ok(hook) => {
            debug!("Hook {hook:X?} set");
            TOWN_HALL_WINDOW_OPEN_HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        Err(_) => {
            return 1;
        }
    }

    debug!("Hooking sidepanel's call to ui_town_hall_window_set_selected_page");
    match hook_call_rel32(
        TOWN_HALL_SIDEPANEL_SET_SELECTED_PAGE_PATCH_OFFSET,
        town_hall_window_set_selected_page_hook as usize as u32,
    ) {
        Ok(hook) => {
            debug!("Hook {hook:X?} set");
            TOWN_HALL_SIDEPANEL_SET_SELECTED_PAGE_HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        Err(_) => {
            return 2;
        }
    }

    debug!("Detouring town hall's rendering function at the selected page switch");
    if deploy_rel32_raw(
        LOAD_TOWN_HALL_SELECTED_PAGE_PATCH_ADDRESS as _,
        (&load_town_hall_selected_page_detour) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 3;
    }

    0
}

#[no_mangle]
unsafe extern "thiscall" fn town_hall_window_open_hook(ui_town_hall_window_address: u32) {
    let orig_address = (*TOWN_HALL_WINDOW_OPEN_HOOK_PTR.load(Ordering::SeqCst)).old_absolute;
    let orig: extern "thiscall" fn(a1: u32) = mem::transmute(orig_address);
    orig(ui_town_hall_window_address);
    crate::handle_open()
}

#[no_mangle]
unsafe extern "thiscall" fn town_hall_window_set_selected_page_hook(ui_town_hall_window_address: u32, page: u32) {
    let orig_address = (*TOWN_HALL_SIDEPANEL_SET_SELECTED_PAGE_HOOK_PTR.load(Ordering::SeqCst)).old_absolute;
    let orig: extern "thiscall" fn(ui_town_hall_window_address: u32, page: u32) = mem::transmute(orig_address);
    orig(ui_town_hall_window_address, page);
    crate::handle_set_selected_page(page)
}

#[no_mangle]
unsafe extern "thiscall" fn town_hall_selected_page_switch_hook() -> i32 {
    crate::handle_selected_page_switch()
}

extern "C" {
    static load_town_hall_selected_page_detour: c_void;
}

global_asm!("
.global {load_town_hall_selected_page_detour}
{load_town_hall_selected_page_detour}:
# save regs
push ecx
push edx

call {town_hall_rendering_hook}

# restore regs
pop edx
pop ecx

jmp [{continuation}]
",
load_town_hall_selected_page_detour = sym load_town_hall_selected_page_detour,
town_hall_rendering_hook = sym town_hall_selected_page_switch_hook,
continuation = sym LOAD_TOWN_HALL_SELECTED_PAGE_CONTINUATION);
