use std::{
    mem::{self},
    panic::{self},
    sync::atomic::{AtomicPtr, Ordering},
};

use hooklet::windows::x86::{hook_function_pointer, FunctionPointerHook};
use log::error;
use p3_api::ui::ui_trading_office_window::UITradingOfficeWindowPtr;

const TOWN_TRADING_OFFICE_CLOSE_POINTER_OFFSET: u32 = UITradingOfficeWindowPtr::VTABLE_OFFSET + 0x118;
static WINDOW_CLOSE_HOOK_PTR: AtomicPtr<FunctionPointerHook> = AtomicPtr::new(std::ptr::null_mut());

#[no_mangle]
unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    panic::set_hook(Box::new(|p| {
        error!("{p}");
    }));

    match hook_function_pointer(TOWN_TRADING_OFFICE_CLOSE_POINTER_OFFSET, window_close_hook as usize as u32) {
        Ok(hook) => {
            WINDOW_CLOSE_HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        Err(_) => return 1,
    }

    0
}

#[no_mangle]
unsafe extern "thiscall" fn window_close_hook(window_address: u32) {
    crate::synchronize_autotrade_settings();
    let orig_address = (*WINDOW_CLOSE_HOOK_PTR.load(Ordering::SeqCst)).old_absolute;
    let orig: extern "thiscall" fn(window_address: u32) = unsafe { mem::transmute(orig_address) };
    orig(window_address)
}
