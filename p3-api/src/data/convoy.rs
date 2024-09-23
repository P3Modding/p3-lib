use super::p3_ptr::P3Pointer;

pub const CONVOY_SIZE: u32 = 0x3c;

pub struct ConvoyPtr {
    pub address: u32,
}

impl ConvoyPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_current_town_index(&self) -> u16 {
        unsafe { self.get(0x39) }
    }

    pub fn get_status(&self) -> u16 {
        unsafe { self.get(0x12) }
    }
}

impl P3Pointer for ConvoyPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
