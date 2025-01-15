use std::{ffi::c_void, thread};

use hooklet::windows::x86::{deploy_rel32_raw, replace_slice_rwx, X86Rel32Type};
use log::{debug, trace};
use p3_api::{mods::init_mod, operations::OperationsPtr};

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    init_mod();

    debug!("Fix execute_operations current ops lock (TODO evaluate whether we have to save and restore regs)");
    match deploy_rel32_raw(0x005468B3, lock_current as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 1,
    }
    match deploy_rel32_raw(0x005468B3 + 5, 0x005468EE, X86Rel32Type::Jump) {
        Ok(_) => {}
        Err(_) => return 2,
    }

    debug!("Fix execute_operations current ops unlock");
    match deploy_rel32_raw(0x00547254, execute_operations_unlock_current as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 3,
    }
    let execute_operations_unlock: [u8; 27] = [
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0x90, 0x90,
    ];
    match replace_slice_rwx(0x547259, &execute_operations_unlock) {
        Ok(_) => {}
        Err(_) => return 4,
    }

    debug!("Fix insert_into_pending_operations_wrapper pending lock (TODO should we keep the >= 52 pending check?)");
    match deploy_rel32_raw(0x0054AA79, lock_pending as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 5,
    }
    match deploy_rel32_raw(0x0054AA79 + 5, 0x0054AAB5, X86Rel32Type::Jump) {
        Ok(_) => {}
        Err(_) => return 6,
    }

    debug!("Fix insert_into_pending_operations_wrapper pending unlock (TODO check reg save)");
    match deploy_rel32_raw(0x0054AAC2, unlock_pending as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 7,
    }
    let insert_into_pending_operations_unlock: [u8; 5] = [0x90, 0x90, 0x90, 0x90, 0x90];
    match replace_slice_rwx(0x0054AAC2 + 5, &insert_into_pending_operations_unlock) {
        Ok(_) => {}
        Err(_) => return 8,
    }

    debug!("Fix operations_network_host_send_to_all_and_move_to_current_ops current lock");
    match deploy_rel32_raw(0x0054BCCB, &lock_current_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 9,
    }
    let operations_network_host_send_to_all_and_move_to_current_ops_lock: [u8; 5] = [0x90, 0x90, 0x90, 0x90, 0x90];
    match replace_slice_rwx(0x0054BCCB + 5, &operations_network_host_send_to_all_and_move_to_current_ops_lock) {
        Ok(_) => {}
        Err(_) => return 10,
    }

    debug!("Fix operations_network_host_send_to_all_and_move_to_current_ops current unlock");
    match deploy_rel32_raw(0x0054BD2C, &unlock_current_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 11,
    }
    match replace_slice_rwx(0x0054BD2C + 5, &[0x90]) {
        Ok(_) => {}
        Err(_) => return 12,
    }

    debug!("Fix operations_network_host_receive_from_all_and_own_pending pending lock");
    // There is a callee saved register restoring `pop ebx`` in the lock code (╯°□°）╯︵ ┻━┻
    match deploy_rel32_raw(0x0054B90D, &lock_pending_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 13,
    }
    match deploy_rel32_raw(0x0054B90D + 5, 0x0054B932, X86Rel32Type::Jump) {
        Ok(_) => {}
        Err(_) => return 14,
    }

    debug!("Fix operations_network_host_receive_from_all_and_own_pending pending unlock");
    match deploy_rel32_raw(0x0054B949, &unlock_pending_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 15,
    }
    // We sneak in the `pop ebx` here to keep the stack intact 🧠
    match replace_slice_rwx(0x0054B949 + 5, &[0x5b, 0x90, 0x90, 0x90, 0x90]) {
        Ok(_) => {}
        Err(_) => return 16,
    }

    debug!("Fix operations_client_receive_from_host current lock");
    match deploy_rel32_raw(0x0054B13F, &try_lock_current_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 17,
    }
    match replace_slice_rwx(0x0054B13F + 5, &[0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90]) {
        Ok(_) => {}
        Err(_) => return 18,
    }
    match deploy_rel32_raw(0x0054B200, &try_lock_current_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 19,
    }
    match replace_slice_rwx(0x0054B200 + 5, &[0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90]) {
        Ok(_) => {}
        Err(_) => return 20,
    }

    debug!("Fix operations_client_receive_from_host current unlock");
    match deploy_rel32_raw(0x0054B193, &unlock_current_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 21,
    }
    match replace_slice_rwx(0x0054B193 + 5, &[0x66, 0x39, 0x7c, 0x24, 0x26, 0x90]) {
        Ok(_) => {}
        Err(_) => return 22,
    }
    // Fix jnz above
    match replace_slice_rwx(0x0054B14D + 1, &[0x49]) {
        Ok(_) => {}
        Err(_) => return 23,
    }
    match deploy_rel32_raw(0x0054B21C, &unlock_current_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 24,
    }
    match replace_slice_rwx(0x0054B21C + 5, &[0x90]) {
        Ok(_) => {}
        Err(_) => return 25,
    }
    // Fix jnz above
    match replace_slice_rwx(0x0054B20E + 1, &[0x12]) {
        Ok(_) => {}
        Err(_) => return 26,
    }

    debug!("Fix operations_network_client_send_pending_operations pending lock");
    match deploy_rel32_raw(0x0054AFB7, &lock_pending_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 27,
    }
    match deploy_rel32_raw(0x0054AFB7 + 5, 0x0054AFD3, X86Rel32Type::Jump) {
        Ok(_) => {}
        Err(_) => return 28,
    }

    debug!("Fix operations_network_client_send_pending_operations pending unlock");
    match deploy_rel32_raw(0x0054B049, &unlock_pending_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 29,
    }
    match deploy_rel32_raw(0x0054B063, &unlock_pending_clean as *const c_void as _, X86Rel32Type::Call) {
        Ok(_) => {}
        Err(_) => return 30,
    }
    match replace_slice_rwx(0x0054B049 + 5, &[0x90]) {
        Ok(_) => {}
        Err(_) => return 31,
    }
    match replace_slice_rwx(0x0054B063 + 5, &[0x90]) {
        Ok(_) => {}
        Err(_) => return 32,
    }

    0
}

#[no_mangle]
unsafe extern "stdcall" fn lock_pending() {
    let thread_id = thread::current().id();
    trace!("lock_pending {thread_id:?}");
    crate::lock(&crate::PENDING_OPS_LOCK);
}

#[no_mangle]
unsafe extern "stdcall" fn try_lock_pending() -> u32 {
    let thread_id = thread::current().id();
    trace!("try_lock_pending {thread_id:?}");
    let in_use = crate::try_lock(&crate::PENDING_OPS_LOCK);
    trace!("try_lock_pending {thread_id:?} {in_use}");
    in_use
}

#[no_mangle]
unsafe extern "stdcall" fn unlock_pending() {
    let operations = OperationsPtr::new();
    // Unlock ops are often not guarded by the mp check, so we have to do an mp check here.
    if operations.get_status() & 0x0c != 0 {
        let thread_id = thread::current().id();
        trace!("unlock_pending {thread_id:?}");
        crate::unlock(&crate::PENDING_OPS_LOCK);
    } else {
        crate::PENDING_OPS_LOCK.store(0, std::sync::atomic::Ordering::SeqCst);
    }
}

#[no_mangle]
unsafe extern "stdcall" fn lock_current() {
    let thread_id = thread::current().id();
    trace!("lock_current {thread_id:?}");
    crate::lock(&crate::CURRENT_OPS_LOCK);
}

#[no_mangle]
unsafe extern "stdcall" fn try_lock_current() -> u32 {
    let thread_id = thread::current().id();
    trace!("try_lock_current {thread_id:?}");
    let in_use = crate::try_lock(&crate::CURRENT_OPS_LOCK);
    trace!("try_lock_current {thread_id:?} {in_use}");
    in_use
}

#[no_mangle]
unsafe extern "stdcall" fn unlock_current() {
    let operations = OperationsPtr::new();
    // Unlock ops are often not guarded by the mp check, so we have to do an mp check here.
    if operations.get_status() & 0x0c != 0 {
        let thread_id = thread::current().id();
        trace!("unlock_current {thread_id:?}");
        crate::unlock(&crate::CURRENT_OPS_LOCK);
    } else {
        crate::CURRENT_OPS_LOCK.store(0, std::sync::atomic::Ordering::SeqCst);
    }
}

#[no_mangle]
unsafe extern "stdcall" fn execute_operations_unlock_current() {
    let operations = OperationsPtr::new();
    let unpacked_traderoute_ptr = operations.get_unpacked_traderoute_ptr();
    operations.set_current_operations_array_pos(0);
    unlock_current();
    if !unpacked_traderoute_ptr.is_null() {
        operations.transfer_loaded_traderoute();
    }
}

macro_rules! save_volatile_registers {
    ($function_name:ident, $symbol_name:ident) => {
        extern "C" {
            static $symbol_name: c_void;
        }

        std::arch::global_asm!("
.global {detour_symbol}
{detour_symbol}:
pushfd
pushad
call {function_symbol}
popad
popfd
ret
",
            detour_symbol = sym $symbol_name,
            function_symbol = sym $function_name,
        );
    };
}

macro_rules! save_volatile_registers_except_eax {
    ($function_name:ident, $symbol_name:ident) => {
        extern "C" {
            static $symbol_name: c_void;
        }

        std::arch::global_asm!("
.global {detour_symbol}
{detour_symbol}:
pushfd
push ecx
push edx
call {function_symbol}
pop edx
pop ecx
popfd
ret
",
            detour_symbol = sym $symbol_name,
            function_symbol = sym $function_name,
        );
    };
}

save_volatile_registers!(lock_pending, lock_pending_clean);
save_volatile_registers_except_eax!(try_lock_pending, try_lock_pending_clean);
save_volatile_registers!(unlock_pending, unlock_pending_clean);

save_volatile_registers!(lock_current, lock_current_clean);
save_volatile_registers_except_eax!(try_lock_current, try_lock_current_clean);
save_volatile_registers!(unlock_current, unlock_current_clean);
