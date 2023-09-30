use crate::tick;
use log::debug;
use p3_api::data::operation::Operation;
use std::mem::transmute;

const CALCULATE_GAME_TIME: u32 = 0x005310D0;
const EXECUTE_OPERATION_ADDRESS: u32 = 0x00535760;
const STATIC_OPERATIONS_ADDRESS: u32 = 0x006DF2F0;
const INSERT_INTO_PENDING_OPERATIONS_WRAPPER_ADDRESS: u32 = 0x0054AA70;
const STATIC_CLASS8: u32 = 0x006DF2F0;

pub extern "thiscall" fn calculate_game_time_hook(game_world: u32) {
    let calculate_game_time_orig: extern "thiscall" fn(game_world: u32) = unsafe { transmute(CALCULATE_GAME_TIME) };
    calculate_game_time_orig(game_world);
    tick();
}

pub fn schedule_operation_raw(op: &[u8]) {
    let insert_into_pending_operations_wrapper: extern "thiscall" fn(operations: u32, input: u32) = unsafe { transmute(CALCULATE_GAME_TIME) };
    insert_into_pending_operations_wrapper(STATIC_OPERATIONS_ADDRESS, op.as_ptr() as u32)
}

pub fn execute_operation(op: &Operation) {
    debug!("execute_operation({:?})", op);
    let op = op.to_raw();
    let execute_operation: extern "thiscall" fn(op: *const u8) = unsafe { transmute(EXECUTE_OPERATION_ADDRESS) };
    execute_operation(op.as_ptr());
}
