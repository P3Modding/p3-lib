use std::mem::transmute;

use log::debug;

use crate::{data::p3_ptr::P3Pointer, operation::Operation};

const OPERATIONS_ADDRESS: u32 = 0x006DF2F0;
pub const OPERATIONS_PTR: OperationsPtr = OperationsPtr::new();
const EXECUTE_OPERATION_ADDRESS: u32 = 0x00535760;

#[derive(Clone, Debug, Copy)]
pub struct OperationsPtr {
    pub address: u32,
}

impl Default for OperationsPtr {
    fn default() -> Self {
        Self::new()
    }
}

impl OperationsPtr {
    pub const fn new() -> Self {
        Self { address: OPERATIONS_ADDRESS }
    }

    pub unsafe fn get_player_merchant_index(&self) -> i32 {
        self.get(0x0924)
    }
}

impl P3Pointer for OperationsPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

pub fn execute_operation(op: &Operation) {
    debug!("execute_operation({:?})", op);
    let op = op.to_raw();
    let execute_operation: extern "thiscall" fn(op: *const u8) = unsafe { transmute(EXECUTE_OPERATION_ADDRESS) };
    execute_operation(op.as_ptr());
}
