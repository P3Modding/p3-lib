use super::{
    enums::TownId,
    town::{TownPtr, TOWN_SIZE},
};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::marker::PhantomData;

pub const GAME_WORLD_ADDRESS: u32 = 0x006DE4A0;

pub struct GameWorldPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> GameWorldPtr<P3> {
    pub fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_town(&self, town: TownId, api: &mut P3) -> Result<TownPtr<P3>, P3ApiError> {
        Ok(TownPtr::new(
            api.read_u32(self.address + 0x68)? + town as u32 * TOWN_SIZE,
        ))
    }
}

impl<P3: P3AccessApi> Default for GameWorldPtr<P3> {
    fn default() -> Self {
        Self::new(GAME_WORLD_ADDRESS)
    }
}
