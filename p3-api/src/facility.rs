use crate::data::p3_ptr::P3Pointer;

pub const FACILITY_SIZE: u32 = 0x10;

#[derive(Clone, Debug)]
pub struct FacilityPtr {
    pub address: u32,
}

impl FacilityPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_employees(&self) -> u16 {
        unsafe { self.get(0x04) }
    }

    pub fn get_type(&self) -> u8 {
        unsafe { self.get(0x06) }
    }

    pub fn get_town_index(&self) -> u8 {
        unsafe { self.get(0x07) }
    }
}

impl P3Pointer for FacilityPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
