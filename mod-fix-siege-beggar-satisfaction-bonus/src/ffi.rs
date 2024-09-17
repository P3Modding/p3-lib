use log::{debug, error};
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
};

const PATCH_ADDRESS: u32 = 0x00629A51;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
    let patch_ptr: *mut u8 = PATCH_ADDRESS as _;

    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(patch_ptr as _, 5, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed: {:?}", error);
        return 1;
    }

    debug!("Patching loop count at {:#x}", PATCH_ADDRESS);
    *patch_ptr = 0x03;

    if !VirtualProtect(patch_ptr as _, 5, old_flags, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed: {:?}", error);
        return 2;
    }

    0
}
