use super::{
    enums::TownId,
    office::{OfficePtr, OFFICE_SIZE},
    p3_ptr::P3Pointer,
    town::{TownPtr, TOWN_SIZE},
};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use log::trace;
use num_traits::FromPrimitive;
use std::marker::PhantomData;

pub const GAME_WORLD_ADDRESS: u32 = 0x006DE4A0;
pub const TICKS_PER_YEAR: u32 = 93440;
pub const TICKS_PER_DAY: u32 = 256;
pub const TICKS_PER_HOUR: f32 = TICKS_PER_DAY as f32 / 24.0;
pub const TICKS_PER_MINUTE: f32 = TICKS_PER_HOUR / 60.0;

#[derive(Clone, Debug)]
pub struct GameWorldPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

#[derive(Clone, Debug)]
pub struct GameWorldTime {
    pub raw: u32,
    pub year: u32,
    pub day_of_year: u32,
    pub hour_of_day: u32,
    pub minute_of_hour: u32,
}

impl<P3: P3AccessApi> GameWorldPtr<P3> {
    pub const fn new() -> Self {
        Self {
            address: GAME_WORLD_ADDRESS,
            api_type: PhantomData,
        }
    }

    pub fn get_offices_count(&self, api: &P3) -> Result<u16, P3ApiError> {
        self.get(0x08, api)
    }

    pub fn get_game_time_raw(&self, api: &P3) -> Result<u32, P3ApiError> {
        api.read_u32(self.address + 0x14)
    }

    pub fn get_game_time(&self, api: &P3) -> Result<GameWorldTime, P3ApiError> {
        let raw = self.get_game_time_raw(api)?;
        let year = raw / TICKS_PER_YEAR;
        let day = 1 + raw / TICKS_PER_DAY;
        let day_of_year = day % 365;
        let tick_of_day = raw & (TICKS_PER_DAY - 1);
        let hour_of_day = (tick_of_day as f32 / TICKS_PER_HOUR) as u32;
        let minute_of_hour = (tick_of_day as f32 / TICKS_PER_MINUTE) as u32 % 60;
        Ok(GameWorldTime {
            raw,
            year,
            day_of_year,
            hour_of_day,
            minute_of_hour,
        })
    }

    pub fn get_raw_town_ids(&self, api: &P3) -> Result<[u8; 40], P3ApiError> {
        self.get(0x18, api)
    }

    pub fn get_raw_town_id(&self, index: u8, api: &P3) -> Result<Option<TownId>, P3ApiError> {
        assert!(index < 40);
        Ok(FromPrimitive::from_u8(self.get(0x18 + index as u32, api)?))
    }

    pub fn get_town(&self, town_index: u8, api: &P3) -> Result<Option<TownPtr<P3>>, P3ApiError> {
        Ok(Some(TownPtr::new(api.read_u32(self.address + 0x68)? + town_index as u32 * TOWN_SIZE)))
    }

    pub fn get_town_by_id(&self, raw_town_id: TownId, api: &P3) -> Result<Option<TownPtr<P3>>, P3ApiError> {
        let raw_ids = self.get_raw_town_ids(api)?;
        for (index, raw_id) in raw_ids.iter().enumerate() {
            if *raw_id == raw_town_id as u8 {
                return Ok(Some(TownPtr::new(api.read_u32(self.address + 0x68)? + index as u32 * TOWN_SIZE)));
            }
        }
        Ok(None)
    }

    pub fn get_town_index(&self, raw_town_id: TownId, api: &P3) -> Result<Option<u8>, P3ApiError> {
        let raw_ids = self.get_raw_town_ids(api)?;
        for (index, raw_id) in raw_ids.iter().enumerate() {
            if *raw_id == raw_town_id as u8 {
                return Ok(Some(index as u8));
            }
        }
        Ok(None)
    }

    pub fn get_office(&self, office_id: u16, api: &P3) -> Result<OfficePtr<P3>, P3ApiError> {
        let base_address: u32 = self.get(0x74, api)?;
        Ok(OfficePtr::new(base_address + office_id as u32 * OFFICE_SIZE))
    }

    pub fn get_office_in_of(&self, town_index: u8, merchant_id: u16, api: &P3) -> Result<Option<OfficePtr<P3>>, P3ApiError> {
        let offices_count = self.get_offices_count(api)?;
        let town = self.get_town(town_index, api)?.unwrap();
        let mut office_id = town.get_first_office_id(api)?;
        loop {
            if office_id >= offices_count {
                return Ok(None);
            }

            let office = self.get_office(office_id, api)?;
            if office.get_merchant_id(api)? == merchant_id {
                trace!("returning office {:#x}", office_id);
                return Ok(Some(office));
            }

            trace!("{:?} belongs to someone else {:#x}", &office, office.get_merchant_id(api)?);
            office_id = office.next_office_id(api)?;
        }
    }
}

impl<P3: P3AccessApi> P3Pointer for GameWorldPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
