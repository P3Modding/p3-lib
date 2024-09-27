use std::{arch::global_asm, ffi::c_void};

use hooklet::X86Rel32Type;
use log::{debug, LevelFilter};

const PATCH_ADDRESS_1: u32 = 0x0061FDDB;
const PATCH_ADDRESS_2: u32 = 0x0061FE8E;

static CONTINUATION_1: u32 = 0x0061FDE2;
static CONTINUATION_2: u32 = 0x0061FE96;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(LevelFilter::Trace);

    if hooklet::deploy_rel32_raw(PATCH_ADDRESS_1 as _, (&fix_index_calc1) as *const _ as _, X86Rel32Type::Jump).is_err() {
        return 1;
    }

    if hooklet::deploy_rel32_raw(PATCH_ADDRESS_2 as _, (&fix_index_calc2) as *const _ as _, X86Rel32Type::Jump).is_err() {
        return 2;
    }

    0
}

#[no_mangle]
pub unsafe extern "cdecl" fn get_correct_index(index: u32) -> u32 {
    let multiplied_index = index * 2;
    let correct_index = match multiplied_index {
        4 => 0,
        _ => 2,
    };
    debug!("Correcting artillery slot index from {multiplied_index} to {correct_index}");

    correct_index
}

extern "C" {
    static fix_index_calc1: c_void;
    static fix_index_calc2: c_void;
}

global_asm!(
    "
.global {fix_index_calc1}
{fix_index_calc1}:
# save regs
push ecx
push edx

push eax
call {get_correct_index}
pop edx

# restore regs
pop edx
pop ecx

jmp [{continuation}]
",
fix_index_calc1 = sym fix_index_calc1,
get_correct_index = sym get_correct_index,
continuation = sym CONTINUATION_1
);

global_asm!(
    "
.global {fix_index_calc2}
{fix_index_calc2}:
# save regs
push eax
push edx

push ecx
call {get_correct_index}
mov ecx, eax
pop edx

# restore regs
pop edx
pop eax

jmp [{continuation}]
",
fix_index_calc2 = sym fix_index_calc2,
get_correct_index = sym get_correct_index,
continuation = sym CONTINUATION_2
);
