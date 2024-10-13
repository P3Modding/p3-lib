use crate::data::p3_ptr::P3Pointer;

pub const TOWN_DATA_ADDRESS: u32 = 0x006DDB90;
pub const TOWN_DATA_SIZE: u32 = 0x34;

#[derive(Debug)]
pub struct StaticTownDataPtr {
    pub address: u32,
}

impl StaticTownDataPtr {
    pub fn new(town_id: u32) -> Self {
        Self {
            address: TOWN_DATA_ADDRESS + town_id * TOWN_DATA_SIZE,
        }
    }

    pub unsafe fn get_anfahrt_x_pos(&self) -> i32 {
        self.get(0x20)
    }

    pub unsafe fn get_anfahrt_y_pos(&self) -> i32 {
        self.get(0x24)
    }
}

impl P3Pointer for StaticTownDataPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
