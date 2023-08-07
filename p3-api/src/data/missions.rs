
use super::{p3_ptr::P3Pointer, storage::StoragePtr, mission::{MISSION_SIZE, MissionPtr}};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::marker::PhantomData;

pub const MISSIONS_ADDRESS: u32 = 0x006DD73C;

#[derive(Debug)]
pub struct MissionsPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> MissionsPtr<P3> {
    pub fn new() -> Self {
        Self {
            address: MISSIONS_ADDRESS,
            api_type: PhantomData,
        }
    }

    pub fn get_mission(&self, mission_id: u16, api: &P3) -> Result<Option<MissionPtr<P3>>, P3ApiError>{
        if mission_id < self.get_missions_size(api)? {
            let base_address: u32 = self.get(0x00, api)?;
            Ok(Some(MissionPtr::new(base_address + mission_id as u32 * MISSION_SIZE)))
        } else {
            Ok(None)
        }
    }

    pub fn get_alderman_mission_id(&self, api: &P3) -> Result<u16, P3ApiError> {
        self.get(0x08, api)
    }

    pub fn get_missions_size(&self, api: &P3) -> Result<u16, P3ApiError> {
        self.get(0x0c, api)
    }
}

impl<P3: P3AccessApi> P3Pointer for MissionsPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
