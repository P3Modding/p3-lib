use log::debug;

use super::enums::WareId;
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::{marker::PhantomData, mem};

pub const TOWN_SIZE: u32 = 0x9F8;

#[derive(Debug)]
pub struct TownPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

#[derive(Debug)]
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

    pub fn get_field_2dc(&self, api: &mut P3) -> Result<u32, P3ApiError> {
        api.read_u32(self.address + 0x2dc)
    }

    pub fn get_field_2e0(&self, api: &mut P3) -> Result<u32, P3ApiError> {
        api.read_u32(self.address + 0x2e0)
    }

    pub fn get_field_0(&self, api: &mut P3) -> Result<Vec<u32>, P3ApiError> {
        let byte_width = 4 * 24;
        let mut input_data: Vec<u8> = vec![0; byte_width];
        debug!("Getting field0 from {:x}", self.address);
        api.read_memory(self.address, &mut input_data)?;
        let mut data: Vec<u32> = Vec::with_capacity(byte_width / 4);
        for i in 0..byte_width / 4 {
            data.push(u32::from_le_bytes(input_data[i * 4..(i * 4) + 4].try_into().unwrap()))
        }
        Ok(data)
    }

    pub fn get_field_3cc(&self, api: &mut P3) -> Result<Vec<u32>, P3ApiError> {
        let byte_width = 4 * 24;
        let mut input_data: Vec<u8> = vec![0; byte_width];
        debug!("Getting field3cc from {:#x}", self.address + 0x3CC);
        api.read_memory(self.address + 0x3CC, &mut input_data)?;
        let mut data: Vec<u32> = Vec::with_capacity(byte_width / 4);
        for i in 0..byte_width / 4 {
            data.push(u32::from_le_bytes(input_data[i * 4..(i * 4) + 4].try_into().unwrap()))
        }
        Ok(data)
    }

    pub fn get_town_ware(&self, ware: WareId, api: &mut P3) -> Result<u32, P3ApiError> {
        api.read_u32(self.address + ware as u32 * 4)
    }

    pub fn get_town_wares(&self, api: &mut P3) -> Result<Vec<u32>, P3ApiError> {
        let wares_count = WareId::Bricks as usize + 1;
        let bytes_len = wares_count * mem::size_of::<u32>();
        let mut input_data: Vec<u8> = vec![0; bytes_len];
        api.read_memory(self.address, &mut input_data)?;
        let mut data: Vec<u32> = Vec::with_capacity(wares_count);
        for i in 0..wares_count {
            data.push(u32::from_le_bytes(input_data[i * 4..(i * 4) + 4].try_into().unwrap()))
        }
        Ok(data)
    }
}
