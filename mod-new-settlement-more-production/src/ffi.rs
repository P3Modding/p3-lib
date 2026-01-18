use log::{debug, error};
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
};

const PATCH_ADDRESS_1: u32 = 0x00532FD9; // iVar11 = 3
const PATCH_ADDRESS_2: u32 = 0x00532F9F; // iVar5 = 3
const PATCH_ADDRESS_3: u32 = 0x00533007; // iVar11 = 4

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);


    // First patch: 0x00532FD9, change BD03 to BD05 (iVar11 = 3 -> iVar11 = 5)
    let patch_ptr_1: *mut u8 = PATCH_ADDRESS_1 as _;
    
    if !VirtualProtect(patch_ptr_1 as _, 5, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed for patch 1: {:?}", error);
        return 1;
    }

    debug!("Patching instruction at {:#x}", PATCH_ADDRESS_1);
    *patch_ptr_1.offset(1) = 0x05; // Change 0x03 to 0x05

    if !VirtualProtect(patch_ptr_1 as _, 5, old_flags, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed for patch 1: {:?}", error);
        return 2;
    }



    // Second patch: 0x00532f9f, change B903 to B904 (iVar11 = 3 -> iVar11 = 4)
    let patch_ptr_2: *mut u8 = PATCH_ADDRESS_2 as _;

    if !VirtualProtect(patch_ptr_2 as _, 5, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed for patch 2: {:?}", error);
        return 3;
    }

    debug!("Patching instruction at {:#x}", PATCH_ADDRESS_2);
    *patch_ptr_2.offset(1) = 0x04; // Change 0x03 to 0x04

    if !VirtualProtect(patch_ptr_2 as _, 5, old_flags, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed for patch 2: {:?}", error);
        return 4;
    }


    
    // Third patch: 0x00533007, change BD040000 to 83C50190 90 (iVar11 = 4 -> iVar11 = iVar11 + 1)
    let patch_ptr_3: *mut u8 = PATCH_ADDRESS_3 as _;

    if !VirtualProtect(patch_ptr_3 as _, 5, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed for patch 3: {:?}", error);
        return 5;
    }

    debug!("Patching instruction at {:#x}", PATCH_ADDRESS_3);
    *patch_ptr_3 = 0x83;           // ADD opcode
    *patch_ptr_3.offset(1) = 0xC5; // EBP register
    *patch_ptr_3.offset(2) = 0x01; // immediate 1
    *patch_ptr_3.offset(3) = 0x90; // NOP padding
    *patch_ptr_3.offset(4) = 0x90; // NOP padding

    if !VirtualProtect(patch_ptr_3 as _, 5, old_flags, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed for patch 3: {:?}", error);
        return 6;
    }

    0
}
