use std::arch::global_asm;

use log::debug;

use crate::run;

const INSERT_INTO_PENDING_OPERATIONS_WRAPPER: u32 = 0x0054AA70;

// This function will be called at 0x00546934 before the original operation_switch (0x00535760).
#[no_mangle] // nomangle should not be required here
extern "C" fn _00535760_hook_handler() {
    run()
}

extern "C" {
    pub fn _00535760_hook(); // This function is defined in assembly. We need a symbol to it to calculate the correct `call` instruction.
}

// Define a function `call_00535760` that calls 00535760 with the right calling convention.
global_asm!(r#"
.global {}
{}:
    # save original regs
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
    ret
"#, sym _00535760_hook, sym _00535760_hook, sym _00535760_hook_handler);

pub unsafe fn schedule_operation(op: &[u8]) {
    debug!("Scheduling {:x?}", op);
    let code: extern "fastcall" fn(class8_ptr: u32, operation_switch_input: *const [u8]) = unsafe { std::mem::transmute(INSERT_INTO_PENDING_OPERATIONS_WRAPPER) };
    (code)(0x006DF2F0, op);
}