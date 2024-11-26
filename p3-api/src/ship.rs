use crate::{
    data::{enums::ShipType, p3_ptr::P3Pointer},
    latin1_to_string,
};

pub const SHIP_SIZE: u32 = 0x180;

#[derive(Debug, Clone, Copy)]
pub struct ShipPtr {
    pub address: u32,
}

impl ShipPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub unsafe fn get_next_ship_index_of_merchant(&self) -> u16 {
        self.get(0x04)
    }

    pub fn get_next_ship_in_convoy(&self) -> u16 {
        unsafe { self.get(0x06) }
    }

    pub fn get_convoy_id(&self) -> u16 {
        unsafe { self.get(0x08) }
    }

    pub fn get_type(&self) -> ShipType {
        unsafe { self.get(0x0e) }
    }

    pub fn get_capacity(&self) -> u32 {
        unsafe { self.get(0x10) }
    }

    pub fn get_max_health(&self) -> u32 {
        unsafe { self.get(0x14) }
    }

    pub fn get_current_health(&self) -> u32 {
        unsafe { self.get(0x18) }
    }

    pub fn get_x(&self) -> i32 {
        unsafe { self.get(0x1c) }
    }

    pub fn get_y(&self) -> i32 {
        unsafe { self.get(0x20) }
    }

    pub unsafe fn get_destination_town_index(&self) -> u8 {
        self.get(0x38)
    }

    pub fn get_last_town_index(&self) -> Option<u8> {
        let town_index: u8 = unsafe { self.get(0x39) };
        if town_index != 0xff {
            Some(town_index)
        } else {
            None
        }
    }

    pub fn get_wares(&self) -> [i32; 24] {
        unsafe { self.get(0x54) }
    }

    pub fn get_status(&self) -> u16 {
        unsafe { self.get(0x134) }
    }

    pub fn get_name(&self) -> String {
        let buf: [u8; 16] = unsafe { self.get(0x160) };
        latin1_to_string(&buf)
    }

    pub unsafe fn calc_free_capacity(&self) -> i32 {
        //TODO weapons, sailors
        self.get_capacity() as i32 - self.get_wares().iter().sum::<i32>() - 10000
    }
}

impl P3Pointer for ShipPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
