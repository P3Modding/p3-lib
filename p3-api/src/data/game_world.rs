use super::{
    enums::TownId,
    town::{TownPtr, TOWN_SIZE},
};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
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
    pub fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_game_time_raw(&self, api: &mut P3) -> Result<u32, P3ApiError> {
        api.read_u32(self.address + 0x14)
    }

    pub fn get_game_time(&self, api: &mut P3) -> Result<GameWorldTime, P3ApiError> {
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

    pub fn get_town(&self, town: TownId, api: &mut P3) -> Result<TownPtr<P3>, P3ApiError> {
        Ok(TownPtr::new(api.read_u32(self.address + 0x68)? + town as u32 * TOWN_SIZE))
    }
}

impl<P3: P3AccessApi> Default for GameWorldPtr<P3> {
    fn default() -> Self {
        Self::new(GAME_WORLD_ADDRESS)
    }
}
