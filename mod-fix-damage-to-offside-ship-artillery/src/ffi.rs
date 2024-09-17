use log::{debug, error};
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
};
const PATCH_ADDRESS: u32 = 0x0067AB30;

static HITBOXES: &[u8] = &[
    0x00, 0x00, 0xC9, 0xFF, 0x13, 0x00, 0xE1, 0xFF, 0x13, 0x00, 0x2F, 0x00, 0xED, 0xFF, 0x2F, 0x00, 0xED, 0xFF, 0xE1, 0xFF, 0x00, 0x00, 0xBE, 0xFF, 0x15, 0x00,
    0xDC, 0xFF, 0x13, 0x00, 0x37, 0x00, 0xED, 0xFF, 0x37, 0x00, 0xEB, 0xFF, 0xDC, 0xFF, 0x00, 0x00, 0xCA, 0xFF, 0x19, 0x00, 0xF2, 0xFF, 0x13, 0x00, 0x43, 0x00,
    0xED, 0xFF, 0x43, 0x00, 0xE7, 0xFF, 0xF2, 0xFF, 0x00, 0x00, 0xBD, 0xFF, 0x16, 0x00, 0xE7, 0xFF, 0x13, 0x00, 0x51, 0x00, 0xED, 0xFF, 0x51, 0x00, 0xEA, 0xFF,
    0xE7, 0xFF,
];

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

    debug!("Patching hitboxes at {:#x}", PATCH_ADDRESS);
    patch_ptr.copy_from(HITBOXES.as_ptr(), HITBOXES.len());

    if !VirtualProtect(patch_ptr as _, 5, old_flags, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed: {:?}", error);
        return 2;
    }

    0
}
