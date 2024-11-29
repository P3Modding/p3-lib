use std::{ffi::c_void, mem::transmute};

use log::debug;

use crate::{data::p3_ptr::P3Pointer, operation::Operation};

pub const OPERATIONS_PTR: OperationsPtr = OperationsPtr::new();
const OPERATIONS_ADDRESS: u32 = 0x006DF2F0;
static EXECUTE_OPERATION_ADDRESS: u32 = 0x00535760;
static ENQUEUE_OPERATION_ADDRESS: u32 = 0x0054AA70;
static ENQUEUE_OPERATION: &extern "thiscall" fn(*mut c_void, *const c_void) = unsafe { transmute(&ENQUEUE_OPERATION_ADDRESS) };

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

    pub unsafe fn enqueue_operation(&self, op: Operation) {
        debug!("Enqueuing operation {op:?}");
        let op_bytes = op.to_raw();
        ENQUEUE_OPERATION(OPERATIONS_ADDRESS as _, op_bytes.as_ptr() as _)
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
