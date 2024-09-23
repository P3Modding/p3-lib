use crate::latin1_to_string;

use super::p3_ptr::P3Pointer;

pub const SHIP_SIZE: u32 = 0x180;

#[derive(Debug)]
pub struct ShipPtr {
    pub address: u32,
}

impl ShipPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_next_ship_in_convoy(&self) -> u16 {
        unsafe { self.get(0x06) }
    }

    pub fn get_convoy_id(&self) -> u16 {
        unsafe { self.get(0x08) }
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

    pub fn get_destination_town_index(&self) -> Option<u8> {
        unsafe { self.get(0x38) }
    }

    pub fn get_last_town_index(&self) -> Option<u8> {
        let town_index: u8 = unsafe { self.get(0x39) };
        if town_index != 0xff {
            Some(town_index)
        } else {
            None
        }
    }

    pub fn get_status(&self) -> u16 {
        unsafe { self.get(0x134) }
    }

    pub fn get_name(&self) -> String {
        let buf: [u8; 16] = unsafe { self.get(0x160) };
        latin1_to_string(&buf)
    }
}

impl P3Pointer for ShipPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
