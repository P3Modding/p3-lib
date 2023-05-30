use super::enums::WareId;
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::{marker::PhantomData, mem};

pub const TOWN_SIZE: u32 = 0x9F8;

pub struct TownPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

pub struct TownDataPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> TownPtr<P3> {
    pub fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_town_data(&self) -> TownDataPtr<P3> {
        TownDataPtr::new(self.address + 0x04)
    }
}

impl<P3: P3AccessApi> TownDataPtr<P3> {
    pub fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_town_ware(&self, ware: WareId, api: &mut P3) -> Result<u32, P3ApiError> {
        api.read_u32(self.address + 0x3CC + ware as u32 * 4)
    }

    pub fn get_town_wares(&self, api: &mut P3) -> Result<Vec<u32>, P3ApiError> {
        let wares_count = WareId::Bricks as usize + 1;
        let bytes_len = wares_count * mem::size_of::<u32>();
        let mut input_data: Vec<u8> = Vec::with_capacity(bytes_len);
        unsafe { input_data.set_len(bytes_len) }
        api.read_memory(self.address + 0x3CC, &mut input_data)?;
        let mut data: Vec<u32> = Vec::with_capacity(wares_count);
        for i in 0..wares_count {
            data.push(u32::from_le_bytes(input_data[i*4..(i*4) + 4].try_into().unwrap()))
        }
        Ok(data)
    }
}