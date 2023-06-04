use std::{marker::PhantomData, mem};

use crate::{p3_access_api::P3AccessApi, P3ApiError};

use super::{p3_ptr::P3Pointer, enums::WareId};

pub const OFFICE_SIZE: u32 = 0x44C;

#[derive(Debug)]
pub struct OfficePtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> OfficePtr<P3> {
    pub fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_ware(&self, ware: WareId, api: &mut P3) -> Result<u32, P3ApiError> {
        api.read_u32(self.address + 0x04 + ware as u32 * 4)
    }

    pub fn get_wares(&self, api: &mut P3) -> Result<Vec<u32>, P3ApiError> {
        let wares_count = WareId::Bricks as usize + 1;
        let bytes_len = wares_count * mem::size_of::<u32>();
        let mut input_data: Vec<u8> = vec![0; bytes_len];
        api.read_memory(self.address + 0x04, &mut input_data)?;
        let mut data: Vec<u32> = Vec::with_capacity(wares_count);
        for i in 0..wares_count {
            data.push(u32::from_le_bytes(input_data[i * 4..(i * 4) + 4].try_into().unwrap()))
        }
        Ok(data)
    }

    pub fn get_merchant_id(&self, api: &mut P3) -> Result<u16, P3ApiError> {
        self.get(0x2c4, api)
    }

    pub fn next_office_id(&self, api: &mut P3) -> Result<u16, P3ApiError> {
        self.get(0x2ca, api)
    }
}

impl<P3: P3AccessApi> P3Pointer for OfficePtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
