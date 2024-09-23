use super::p3_ptr::P3Pointer;

pub const MISSION_SIZE: u32 = 0x18;

#[derive(Debug)]
pub struct MissionPtr {
    pub address: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct NewSettlementMission {
    pub productivities_effective: u32,
    pub raw_town_id: u8,
    pub u1: u8,
    pub u2: u8,
    pub u3: u8,
}

#[derive(Debug)]
pub enum Mission {
    NewSettlement(NewSettlementMission),
}

impl MissionPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_next_mission_id(&self) -> u16 {
        unsafe { self.get(0x04) }
    }

    pub fn get_unknown(&self) -> u16 {
        unsafe { self.get(0x06) }
    }

    pub fn get_end_date(&self) -> u32 {
        unsafe { self.get(0x08) }
    }

    pub fn get_type(&self) -> u8 {
        unsafe { self.get(0x0c) }
    }

    pub fn get_mission_data(&self) -> Mission {
        let mission_type = self.get_type();
        match mission_type {
            0 => Mission::NewSettlement(unsafe { self.get(0x10) }),
            x => unimplemented!("{}", x),
        }
    }
}

impl P3Pointer for MissionPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
