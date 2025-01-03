use std::{arch::global_asm, ffi::c_void};

use hooklet::windows::x86::{deploy_rel32_raw, replace_slice_rwx, X86Rel32Type};
use log::debug;
use p3_api::{game_world::GAME_WORLD_PTR, mods};

const COUNT_BRIBED_COUNCILLORS_PATCH_ADDRESS: u32 = 0x0053AC99;
static COUNT_BRIBED_COUNCILLORS_CONTINUATION: u32 = 0x0053ACA6;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    mods::init_mod();

    debug!("Properly set the councillor's briber to -1 if the bribe failed");
    match replace_slice_rwx(0x0053AD85, &[0xc6, 0x01, 0xff]) {
        Ok(_) => {}
        Err(_) => return 1,
    }

    debug!("Reset annoyed councillors when opening the bath house");
    match replace_slice_rwx(0x005B17B7, &[0x90, 0x90]) {
        Ok(_) => {}
        Err(_) => return 2,
    }

    debug!("Fix the limit of 2 bribed councillors");
    match deploy_rel32_raw(
        COUNT_BRIBED_COUNCILLORS_PATCH_ADDRESS,
        (&count_bribed_councillors_detour) as *const _ as _,
        X86Rel32Type::Jump,
    ) {
        Ok(_) => {}
        Err(_) => return 3,
    }

    0
}

#[no_mangle]
unsafe extern "thiscall" fn count_bribed_councillors(merchant_index: u8, town_index: u8) -> i32 {
    debug!("count_bribed_councillors(merchant_index={merchant_index:#x}, town_index={town_index:#x})");
    let town = GAME_WORLD_PTR.get_town(town_index);
    let mut bribed = 0;
    let bribers = town.get_councillor_bribes();
    for briber in bribers {
        if briber == merchant_index {
            bribed += 1;
        }
    }

    debug!("Found {bribed} local councillors already bribed by the merchant");
    bribed
}

extern "C" {
    static count_bribed_councillors_detour: c_void;
}

global_asm!("
.global {count_bribed_councillors_detour}
{count_bribed_councillors_detour}:
# save regs except eax and edx (not read)
push ecx

push [esi+0x10]
call {count_bribed_councillors}
mov edi, eax

# restore regs
pop ecx

jmp [{continuation}]
",
count_bribed_councillors_detour = sym count_bribed_councillors_detour,
count_bribed_councillors = sym count_bribed_councillors,
continuation = sym COUNT_BRIBED_COUNCILLORS_CONTINUATION);
