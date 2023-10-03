use crate::tick;
use log::debug;
use p3_api::{data::{operation::Operation, game_world::GameWorldPtr, class6::Class6Ptr}, p3_access_api::raw_p3_access_api::RawP3AccessApi};
use std::mem::transmute;

const HANDLE_CLASS11_TICK: u32 = 0x006127C0;
const EXECUTE_OPERATION_ADDRESS: u32 = 0x00535760;
const STATIC_OPERATIONS_ADDRESS: u32 = 0x006DF2F0;
const INSERT_INTO_PENDING_OPERATIONS_WRAPPER_ADDRESS: u32 = 0x0054AA70;
const DO_NOTIFICATION_ADDRESS: u32 = 0x00548CA0;
pub const P3: RawP3AccessApi = RawP3AccessApi::new();
pub const GAME_WORLD: GameWorldPtr<RawP3AccessApi> = GameWorldPtr::new();
pub const CLASS6: Class6Ptr<RawP3AccessApi> = Class6Ptr::new();

pub extern "thiscall" fn handle_class11_tick_hook(class11: u32, unknown: u8) {
    let calculate_game_time_orig: extern "thiscall" fn(class11: u32, unknown: u8) = unsafe { transmute(HANDLE_CLASS11_TICK) };
    calculate_game_time_orig(class11, unknown);
    tick();
}

pub extern "thiscall" fn handle_ship_docked_do_notification_wrapper_hook(operations: u32, merchant_index: u16, _a3: u8, a4: i32, town_index: i32, string_index: u32, index: u16) {
    debug!("handle_ship_docked_do_notification_wrapper_hook()");
    let do_notification: extern "thiscall" fn(operations: u32, merchant_index: u16, a3: u8, a4: i32, town_index: i32, string_index: u32, index: u16) = unsafe { transmute(DO_NOTIFICATION_ADDRESS) };
    do_notification(operations, merchant_index, 0xff, a4, town_index, string_index, index);
}

pub extern "thiscall" fn handle_convoy_docked_do_notification_wrapper_hook(operations: u32, merchant_index: u16, _a3: u8, a4: i32, town_index: i32, string_index: u32, index: u16) {
    debug!("handle_convoy_docked_do_notification_wrapper_hook()");
    let do_notification: extern "thiscall" fn(operations: u32, merchant_index: u16, a3: u8, a4: i32, town_index: i32, string_index: u32, index: u16) = unsafe { transmute(DO_NOTIFICATION_ADDRESS) };
    do_notification(operations, merchant_index, 0xff, a4, town_index, string_index, index);
}

pub extern "thiscall" fn handle_repair_complete_do_notification_wrapper_hook(operations: u32, merchant_index: u16, _a3: u8, a4: i32, town_index: i32, string_index: u32, index: u16) {
    debug!("handle_repair_complete_do_notification_wrapper_hook()");
    let do_notification: extern "thiscall" fn(operations: u32, merchant_index: u16, a3: u8, a4: i32, town_index: i32, string_index: u32, index: u16) = unsafe { transmute(DO_NOTIFICATION_ADDRESS) };
    do_notification(operations, merchant_index, 0xff, a4, town_index, string_index, index);
}

pub fn schedule_operation_raw(op: &[u8]) {
    let insert_into_pending_operations_wrapper: extern "thiscall" fn(operations: u32, input: u32) = unsafe { transmute(INSERT_INTO_PENDING_OPERATIONS_WRAPPER_ADDRESS) };
    insert_into_pending_operations_wrapper(STATIC_OPERATIONS_ADDRESS, op.as_ptr() as u32)
}

pub fn execute_operation(op: &Operation) {
    debug!("execute_operation({:?})", op);
    let op = op.to_raw();
    let execute_operation: extern "thiscall" fn(op: *const u8) = unsafe { transmute(EXECUTE_OPERATION_ADDRESS) };
    execute_operation(op.as_ptr());
}
