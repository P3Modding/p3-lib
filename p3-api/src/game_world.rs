use log::trace;
use num_traits::FromPrimitive;

use crate::{
    data::{
        enums::TownId,
        office::{OfficePtr, OFFICE_SIZE},
        p3_ptr::P3Pointer,
    },
    merchant::{MerchantPtr, MERCHANT_SIZE},
    town::{TownPtr, TOWN_SIZE},
};

pub const GAME_WORLD_PTR: GameWorldPtr = GameWorldPtr::new();
pub const GAME_WORLD_ADDRESS: u32 = 0x006DE4A0;
pub const TICKS_PER_YEAR: u32 = 93440;
pub const TICKS_PER_DAY: u32 = 256;
pub const TICKS_PER_HOUR: f32 = TICKS_PER_DAY as f32 / 24.0;
pub const TICKS_PER_MINUTE: f32 = TICKS_PER_HOUR / 60.0;

#[derive(Clone, Debug)]
pub struct GameWorldPtr {
    pub address: u32,
}

#[derive(Clone, Debug)]
pub struct GameWorldTime {
    pub raw: u32,
    pub year: u32,
    pub day_of_year: u32,
    pub hour_of_day: u32,
    pub minute_of_hour: u32,
}

impl Default for GameWorldPtr {
    fn default() -> Self {
        Self::new()
    }
}

impl GameWorldPtr {
    pub const fn new() -> Self {
        Self { address: GAME_WORLD_ADDRESS }
    }

    pub unsafe fn get_month(&self) -> u8 {
        self.get(0x01)
    }

    pub fn get_offices_count(&self) -> u16 {
        unsafe { self.get(0x08) }
    }

    pub unsafe fn get_towns_count(&self) -> u16 {
        self.get(0x10)
    }

    pub fn get_game_time_raw(&self) -> u32 {
        unsafe { self.get(0x14) }
    }

    pub fn get_game_time(&self) -> GameWorldTime {
        let raw = self.get_game_time_raw();
        let year = raw / TICKS_PER_YEAR;
        let day = 1 + raw / TICKS_PER_DAY;
        let day_of_year = day % 365;
        let tick_of_day = raw & (TICKS_PER_DAY - 1);
        let hour_of_day = (tick_of_day as f32 / TICKS_PER_HOUR) as u32;
        let minute_of_hour = (tick_of_day as f32 / TICKS_PER_MINUTE) as u32 % 60;
        GameWorldTime {
            raw,
            year,
            day_of_year,
            hour_of_day,
            minute_of_hour,
        }
    }

    pub fn get_raw_town_ids(&self) -> [TownId; 40] {
        unsafe { self.get(0x18) }
    }

    pub fn get_town_id(&self, index: u8) -> Option<TownId> {
        assert!(index < 40);
        FromPrimitive::from_u8(unsafe { self.get(0x18 + index as u32) })
    }

    pub fn get_town(&self, town_index: u8) -> TownPtr {
        let towns_address: u32 = unsafe { self.get(0x68) };
        TownPtr::new(towns_address + town_index as u32 * TOWN_SIZE)
    }

    pub fn get_town_by_id(&self, town_id: TownId) -> Option<TownPtr> {
        let towns_address: u32 = unsafe { self.get(0x68) };
        let raw_ids = self.get_raw_town_ids();
        for (index, raw_id) in raw_ids.iter().enumerate() {
            if *raw_id == town_id {
                return Some(TownPtr::new(towns_address + index as u32 * TOWN_SIZE));
            }
        }
        None
    }

    pub fn get_town_index(&self, raw_town_id: TownId) -> Option<u8> {
        let raw_ids = self.get_raw_town_ids();
        for (index, raw_id) in raw_ids.iter().enumerate() {
            if *raw_id == raw_town_id {
                return Some(index as u8);
            }
        }
        None
    }

    pub fn get_office(&self, office_index: u16) -> OfficePtr {
        let base_address: u32 = unsafe { self.get(0x74) };
        OfficePtr::new(base_address + office_index as u32 * OFFICE_SIZE)
    }

    pub unsafe fn get_merchant(&self, index: u16) -> MerchantPtr {
        let base_address: u32 = self.get(0x78);
        MerchantPtr::new(base_address + index as u32 * MERCHANT_SIZE)
    }

    pub unsafe fn get_office_in_of(&self, town_index: u8, merchant_id: u16) -> Option<OfficePtr> {
        let offices_count = self.get_offices_count();
        let town = self.get_town(town_index);
        let mut office_id = town.get_first_office_index();
        loop {
            if office_id >= offices_count {
                return None;
            }

            let office = self.get_office(office_id);
            if office.get_merchant_id() == merchant_id {
                trace!("returning office {:#x}", office_id);
                return Some(office);
            }

            trace!("{:?} belongs to someone else {:#x}", &office, office.get_merchant_id());
            office_id = office.next_office_in_town_index();
        }
    }

    pub unsafe fn find_town_index(&self, town_id: TownId) -> Option<u8> {
        let raw_ids = self.get_raw_town_ids();
        for (index, id) in raw_ids.iter().enumerate() {
            if *id == town_id {
                return Some(index as _);
            }
        }
        None
    }

    pub unsafe fn find_town_id(&self, town_index: u8) -> Option<TownId> {
        let raw_ids = self.get_raw_town_ids();
        if town_index < self.get_towns_count() as u8 {
            Some(raw_ids[town_index as usize])
        } else {
            None
        }
    }
}

impl P3Pointer for GameWorldPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
