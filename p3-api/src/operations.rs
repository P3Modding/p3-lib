use std::mem::transmute;

use log::debug;

use crate::operation::Operation;

const EXECUTE_OPERATION_ADDRESS: u32 = 0x00535760;
//const STATIC_OPERATIONS_ADDRESS: u32 = 0x006DF2F0;

pub fn execute_operation(op: &Operation) {
    debug!("execute_operation({:?})", op);
    let op = op.to_raw();
    let execute_operation: extern "thiscall" fn(op: *const u8) = unsafe { transmute(EXECUTE_OPERATION_ADDRESS) };
    execute_operation(op.as_ptr());
}
