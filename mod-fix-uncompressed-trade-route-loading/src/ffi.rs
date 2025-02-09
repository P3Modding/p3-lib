use std::ffi::c_void;

use hooklet::windows::x86::{deploy_rel32_raw, X86Rel32Type};

const PATCH_ADDRESS: u32 = 0x004D5FD5;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    match deploy_rel32_raw(PATCH_ADDRESS, &abs_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 1,
    }

    0
}

#[no_mangle]
unsafe extern "thiscall" fn abs(input: i32) -> i32 {
    input.abs()
}

extern "C" {
    static abs_clean: c_void;
}

std::arch::global_asm!("
.global {detour_symbol}
{detour_symbol}:
pushfd
push ecx
push edx
mov ecx, eax
call {function_symbol}
pop edx
pop ecx
popfd
ret
",
    detour_symbol = sym abs_clean,
    function_symbol = sym abs,
);
