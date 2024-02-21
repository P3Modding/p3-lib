use super::p3_ptr::P3Pointer;
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use num_traits::FromPrimitive;
use std::marker::PhantomData;

pub const CONVOY_SIZE: u32 = 0x3c;

pub struct ConvoyPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> ConvoyPtr<P3> {
    pub fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_current_town_index(&self, api: &P3) -> Result<Option<u16>, P3ApiError> {
        let raw_town_id: u8 = self.get(0x39, api)?;
        Ok(FromPrimitive::from_u8(raw_town_id))
    }

    pub fn get_status(&self, api: &P3) -> Result<u16, P3ApiError> {
        let raw_status = self.get(0x12, api)?;
        Ok(raw_status)
    }
}

impl<P3: P3AccessApi> P3Pointer for ConvoyPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
