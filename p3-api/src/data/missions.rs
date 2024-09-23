#![allow(clippy::new_without_default)]
use super::{
    mission::{MissionPtr, MISSION_SIZE},
    p3_ptr::P3Pointer,
};

pub const MISSIONS_ADDRESS: u32 = 0x006DD73C;

#[derive(Debug)]
pub struct MissionsPtr {
    pub address: u32,
}

impl MissionsPtr {
    pub fn new() -> Self {
        Self { address: MISSIONS_ADDRESS }
    }

    pub fn get_mission(&self, mission_id: u16) -> Option<MissionPtr> {
        if mission_id < self.get_missions_size() {
            let base_address: u32 = unsafe { self.get(0x00) };
            Some(MissionPtr::new(base_address + mission_id as u32 * MISSION_SIZE))
        } else {
            None
        }
    }

    pub fn get_alderman_mission_id(&self) -> u16 {
        unsafe { self.get(0x08) }
    }

    pub fn get_missions_size(&self) -> u16 {
        unsafe { self.get(0x0c) }
    }
}

impl P3Pointer for MissionsPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
