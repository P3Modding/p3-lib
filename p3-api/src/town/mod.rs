use log::debug;
use map::TownMapPtr;
use shipyard::{ShipLevels, ShipyardPtr};

use crate::{
    data::{enums::TownId, p3_ptr::P3Pointer, storage::StoragePtr},
    facility::{FacilityPtr, FACILITY_SIZE},
};

pub mod map;
pub mod shipyard;

pub const TOWN_SIZE: u32 = 0x9F8;

#[derive(Debug)]
pub struct TownPtr {
    pub address: u32,
}

impl TownPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_storage(&self) -> StoragePtr {
        StoragePtr::new(self.address)
    }

    pub fn get_raw_town_id(&self) -> TownId {
        //TODO enforce enum variant
        unsafe { self.get(0x2c1) }
    }

    pub fn get_daily_consumptions_citizens(&self) -> [i32; 24] {
        unsafe { self.get(0x310) }
    }

    pub fn get_first_office_index(&self) -> u16 {
        debug!("town {:#X} getting first office index", self.address);
        unsafe { self.get(0x784) }
    }

    pub fn get_town_map(&self) -> TownMapPtr {
        TownMapPtr { address: self.address + 0x7a4 }
    }

    pub fn get_shipyard(&self) -> ShipyardPtr {
        ShipyardPtr::new(self.address + 0x810)
    }

    pub fn get_build_ship_capacity_markup(&self) -> f32 {
        unsafe { self.get(0x818) }
    }

    pub fn get_build_ship_quality_levels(&self) -> ShipLevels {
        unsafe { self.get(0x824) }
    }

    pub fn get_build_ship_828_always_0(&self) -> [u8; 4] {
        unsafe { self.get(0x828) }
    }

    pub fn get_facility(&self, index: u32) -> FacilityPtr {
        FacilityPtr::new(self.address + 0x840 + FACILITY_SIZE * index)
    }
}

impl P3Pointer for TownPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
