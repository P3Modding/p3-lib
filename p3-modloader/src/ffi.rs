use std::mem;
use std::sync::atomic::{AtomicPtr, Ordering};

use hooklet::{hook_call_rel32, CallRel32Hook};
use log::{debug, error};
use windows::core::PSTR;
use windows::s;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

const WINMAIN_ADDRESS: u32 = 0x0064BE10;
static HOOK_PTR: AtomicPtr<CallRel32Hook> = AtomicPtr::new(std::ptr::null_mut());

#[no_mangle]
extern "system" fn DllMain(_hist_dll: *const u8, fdw_reason: u32, _lpv_reserved: *const u8) -> u32 {
    if fdw_reason == DLL_PROCESS_ATTACH {
        let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
        log::set_max_level(log::LevelFilter::Trace);
        debug!("DllMain patching WinMain call");
        unsafe {
            let hook = match hook_call_rel32(s!("Patrician3_patched.exe"), 0x0023CA22, WinMain_hook as usize) {
                Ok(hook) => hook,
                Err(e) => {
                    error!("Failed to set hook: {:?}", e);
                    return 1;
                }
            };
            debug!("Storing hook");
            HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        debug!("DllMain finished successfully")
    }
    1
}

#[no_mangle]
extern "stdcall" fn WinMain_hook(h_instance: HMODULE, h_prev_instance: HMODULE, lp_cmd_line: PSTR, n_show_cmd: u32) -> i32 {
    debug!("WinMain_hook starting");
    let original_winmain: extern "stdcall" fn(h_instance: HMODULE, h_prev_instance: HMODULE, lp_cmd_line: PSTR, n_show_cmd: u32) -> i32 =
        unsafe { mem::transmute(WINMAIN_ADDRESS) };
    crate::load();
    debug!("WinMain_hook calling original main");
    original_winmain(h_instance, h_prev_instance, lp_cmd_line, n_show_cmd)
}
