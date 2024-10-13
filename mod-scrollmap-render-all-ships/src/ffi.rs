use std::{arch::global_asm, ffi::c_void};

use log::{debug, error, LevelFilter};
use p3_api::ships::ShipsPtr;
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
};

const PATCH_ADDRESS: u32 = 0x00451759;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(LevelFilter::Trace);
    let trampoline_address: u32 = (&draw_all_ships_trampoline) as *const _ as _;
    let relative_trampoline_address: u32 = trampoline_address.wrapping_sub(PATCH_ADDRESS) - 5; //TODO move pointer calcs to hooklet

    let mut patch: [u8; 5] = [0; 5];
    patch[0] = 0xe9;
    patch[1..5].copy_from_slice(&relative_trampoline_address.to_le_bytes());
    let patch_ptr: *mut u8 = PATCH_ADDRESS as _;

    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(patch_ptr as _, 5, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed: {:?}", error);
        return 1;
    }

    debug!("Activating trampoline at {:x?}", trampoline_address as *const u32);
    patch_ptr.copy_from(patch.as_ptr(), patch.len());

    if !VirtualProtect(patch_ptr as _, 5, old_flags, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed: {:?}", error);
        return 2;
    }

    0
}

#[no_mangle]
pub unsafe extern "cdecl" fn fixup_all_ships(spotted_y: *mut i32, spotted_index: *mut u32) -> u32 {
    let ships = ShipsPtr::new();
    let mut new_spotted_size = 0;
    for i in 0..ships.get_ships_size() {
        let ship = ships.get_ship(i).unwrap();
        let ship_status = ship.get_status();

        if ship_status != 0x12 && ship_status != 0x0f {
            continue;
        }

        *spotted_y.add(new_spotted_size as _) = ship.get_y() >> 16;
        *spotted_index.add(new_spotted_size as _) = i as _;
        new_spotted_size += 1;
    }

    new_spotted_size
}

extern "C" {
    static draw_all_ships_trampoline: c_void;
}

global_asm!(
    "
.global {draw_all_ships_trampoline}
{draw_all_ships_trampoline}:
# save regs
push eax
push ecx
push edx

# call fixup_all_ships
mov eax, [esp+0x30]
mov ecx, [esp+0x2C]
push ecx
push eax
call {fixup_all_ships}
mov ebp, eax
pop eax
pop eax

# restore regs
pop edx
pop ecx
pop eax

# jump to second part of draw_spotted_ships
mov eax, 0x00451B58
jmp eax",
draw_all_ships_trampoline = sym draw_all_ships_trampoline,
fixup_all_ships = sym fixup_all_ships
);
