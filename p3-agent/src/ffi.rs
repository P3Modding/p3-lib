use crate::tick;
use log::debug;
use p3_api::{data::{operation::Operation}};
use std::arch::{asm, global_asm};

const INSERT_INTO_PENDING_OPERATIONS_WRAPPER: u32 = 0x0054AA70;
const STATIC_CLASS8: u32 = 0x006DF2F0;

// This function will be called at 0x00546934 before the original operation_switch (0x00535760).
extern "C" fn _00535760_hook_handler() {
    tick()
}

extern "C" {
    pub fn _00535760_hook(); // This function is defined in assembly. We need a symbol to it to calculate the correct `call` instruction.
}

// Define a function `_00535760_hook` that calls 00535760 with the right calling convention.
global_asm!(r#"
.global {}
{}:
    # save original regs
    push eax
    push edi
    push dx
    push ecx

    # call _00535760_hook_handler
    call {}

    # pop original regs
    pop ecx
    pop dx
    pop edi

    # call original function and return
    mov eax, 0x00535760
    call eax
    pop eax
    ret
"#, sym _00535760_hook, sym _00535760_hook, sym _00535760_hook_handler);

pub fn schedule_operation_raw(op: &[u8]) {
    unsafe {
        debug!("Scheduling {:x?}", op);
        // rustc can't do thiscall because reasons
        asm!(
            "push eax",
            "call ebx", //TODO properly define clobber by this thiscall call
            in("eax") op.as_ptr(),
            in("ebx") INSERT_INTO_PENDING_OPERATIONS_WRAPPER,
            in("ecx") STATIC_CLASS8,
        );

        debug!("Scheduling done");
    }
}

pub fn schedule_operation(op: &Operation) {
    unsafe {
        let op = op.to_raw();
        // rustc can't do thiscall because reasons
        asm!(
            "push eax",
            "call ebx", //TODO properly define clobber by this thiscall call
            in("eax") op.as_ptr(),
            in("ebx") INSERT_INTO_PENDING_OPERATIONS_WRAPPER,
            in("ecx") STATIC_CLASS8,
        );
    }
}
