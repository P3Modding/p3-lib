use std::marker::PhantomData;
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use super::p3_ptr::P3Pointer;

pub const MISSION_SIZE: u32 = 0x18;

#[derive(Debug)]
pub struct MissionPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

#[derive(Debug)]
#[repr(C)]
pub struct NewSettlementMission {
    pub productivities_effective: u32,
    pub raw_town_id: u8,
    pub u1: u8,
    pub u2: u8,
    pub u3: u8,
}

#[derive(Debug)]
pub enum Mission {
    NewSettlement(NewSettlementMission),
}

impl<P3: P3AccessApi> MissionPtr<P3> {
    pub fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_next_mission_id(&self, api: &P3) -> Result<u16, P3ApiError> {
        self.get(0x04, api)
    }

    pub fn get_unknown(&self, api: &P3) -> Result<u16, P3ApiError> {
        self.get(0x06, api)
    }

    pub fn get_end_date(&self, api: &P3) -> Result<u32, P3ApiError> {
        self.get(0x08, api)
    }

    pub fn get_type(&self, api: &P3) -> Result<u8, P3ApiError> {
        self.get(0x0c, api)
    }

    pub fn get_mission_data(&self, api: &P3) -> Result<Mission, P3ApiError> {
        let mission_type = self.get_type(api)?;
        match mission_type {
            0 => Ok(Mission::NewSettlement(self.get(0x10, api)?)),
            x => unimplemented!("{}", x),
        }
    }
}

impl<P3: P3AccessApi> P3Pointer for MissionPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
