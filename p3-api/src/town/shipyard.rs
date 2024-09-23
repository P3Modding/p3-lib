use crate::data::p3_ptr::P3Pointer;

#[derive(Debug)]
#[repr(C)]
pub struct ShipLevels {
    pub snaikka_level: i8,
    pub craier_level: i8,
    pub cog_level: i8,
    pub holk_level: i8,
}

#[derive(Debug)]
pub struct ShipyardPtr {
    pub address: u32,
}

impl ShipyardPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_experience(&self) -> u32 {
        unsafe { self.get(0x00) }
    }

    pub fn get_pending_experience(&self) -> u32 {
        unsafe { self.get(0x04) }
    }

    pub fn get_utilization_markup(&self) -> f32 {
        unsafe { self.get(0x08) }
    }

    pub fn get_building_ship_index(&self) -> i16 {
        unsafe { self.get(0x0c) }
    }

    pub fn get_repairing_ship_index(&self) -> i16 {
        unsafe { self.get(0x0e) }
    }

    pub fn get_current_quality_levels(&self) -> ShipLevels {
        unsafe { self.get(0x14) }
    }

    pub fn get_always_zero(&self) -> [u8; 4] {
        unsafe { self.get(0x18) }
    }
}

impl P3Pointer for ShipyardPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
