use map::TownMapPtr;
use shipyard::ShipyardPtr;

use crate::{
    data::{enums::TownId, p3_ptr::P3Pointer, storage::StoragePtr},
    facility::{FacilityPtr, FACILITY_SIZE},
    latin1_ptr_to_string,
};

pub mod map;
pub mod shipyard;
pub mod static_town_data;

pub const TOWN_SIZE: u32 = 0x9F8;
pub const TOWN_NAME_PTRS_ADDRESS: u32 = 0x006DDA00;
pub const WARE_BASE_PRICES: *const f32 = 0x00673A18 as _;

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

    pub unsafe fn get_town_id(&self) -> TownId {
        self.get(0x2c1)
    }

    pub unsafe fn get_town_id_u8(&self) -> u8 {
        self.get(0x2c1)
    }

    pub fn get_daily_consumptions_citizens(&self) -> [i32; 24] {
        unsafe { self.get(0x310) }
    }

    pub fn get_production_values(&self) -> [i32; 24] {
        unsafe { self.get(0x490) }
    }

    pub fn get_unknown_stock(&self) -> [i32; 24] {
        unsafe { self.get(0x670) }
    }

    pub fn get_councillor_bribes(&self) -> [u8; 4] {
        unsafe { self.get(0x6dc) }
    }

    pub unsafe fn get_first_office_index(&self) -> u16 {
        self.get(0x784)
    }

    pub fn get_town_map(&self) -> TownMapPtr {
        TownMapPtr { address: self.address + 0x7a4 }
    }

    pub fn get_shipyard(&self) -> ShipyardPtr {
        ShipyardPtr::new(self.address + 0x810)
    }

    pub fn get_facility(&self, index: u32) -> FacilityPtr {
        FacilityPtr::new(self.address + 0x840 + FACILITY_SIZE * index)
    }

    pub unsafe fn get_price_thresholds(&self) -> [[i32; 4]; 24] {
        self.get(0x4f0)
    }
}

impl P3Pointer for TownPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

pub fn get_town_name(town_index: u8) -> Option<String> {
    unsafe {
        let town_names_ptr: *const *const u8 = TOWN_NAME_PTRS_ADDRESS as _;
        let town_name_ptr = *town_names_ptr.add(town_index as _);
        if town_name_ptr.is_null() {
            None
        } else {
            Some(latin1_ptr_to_string(town_name_ptr))
        }
    }
}
