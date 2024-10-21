use crate::{
    data::{enums::TownId, p3_ptr::P3Pointer},
    Point,
};

pub const TOWN_DATA_ADDRESS: u32 = 0x006DDB90;
pub const TOWN_DATA_SIZE: u32 = 0x34;

#[derive(Debug)]
pub struct StaticTownDataPtr {
    pub address: u32,
}

impl StaticTownDataPtr {
    pub fn new(town_id: TownId) -> Self {
        Self {
            address: TOWN_DATA_ADDRESS + town_id as u32 * TOWN_DATA_SIZE,
        }
    }

    pub unsafe fn get_anfahrt_x_pos(&self) -> i32 {
        self.get(0x20)
    }

    pub unsafe fn get_anfahrt_y_pos(&self) -> i32 {
        self.get(0x24)
    }

    pub unsafe fn get_point_i16(&self) -> Point<i16> {
        Point::new(self.get_anfahrt_x_pos() as _, self.get_anfahrt_y_pos() as _)
    }
}

impl P3Pointer for StaticTownDataPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
