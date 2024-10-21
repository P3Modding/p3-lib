use std::mem;

use scheduled_task::{ScheduledTaskPtr, SCHEDULED_TASK_SIZE};

use crate::data::p3_ptr::P3Pointer;
pub mod scheduled_task;

const SCHEDULED_TASKS_ADDRESS: u32 = 0x006DD73C;
pub const SCHEDULED_TASKS_PTR: ScheduledTasksPtr = ScheduledTasksPtr::new();

#[derive(Clone, Debug, Copy)]
pub struct ScheduledTasksPtr {
    pub address: u32,
}

impl Default for ScheduledTasksPtr {
    fn default() -> Self {
        Self::new()
    }
}

impl ScheduledTasksPtr {
    pub const fn new() -> Self {
        Self {
            address: SCHEDULED_TASKS_ADDRESS,
        }
    }

    pub unsafe fn get_scheduled_task(&self, index: u16) -> ScheduledTaskPtr {
        let base_address: u32 = unsafe { self.get(0x00) };
        ScheduledTaskPtr::new(base_address + index as u32 * SCHEDULED_TASK_SIZE)
    }

    pub unsafe fn get_merchant_alderman_mission_task_index(&self, merchant_index: i32) -> i16 {
        let func: extern "thiscall" fn(scheduled_tasks: u32, merchant_index: i32) -> i16 = mem::transmute(0x004EC694);
        func(self.address, merchant_index)
    }
}

impl P3Pointer for ScheduledTasksPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
