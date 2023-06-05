use super::{
    enums::{ShipWeaponId, WareId},
    p3_ptr::P3Pointer,
};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::{marker::PhantomData, mem};

#[derive(Debug)]
pub struct StoragePtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> StoragePtr<P3> {
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
        let wares_count = WareId::Carbine as usize + 1;
        let bytes_len = wares_count * mem::size_of::<u32>();
        let mut input_data: Vec<u8> = vec![0; bytes_len];
        api.read_memory(self.address + 0x04, &mut input_data)?;
        let mut data: Vec<u32> = Vec::with_capacity(wares_count);
        for i in 0..wares_count {
            data.push(u32::from_le_bytes(input_data[i * 4..(i * 4) + 4].try_into().unwrap()))
        }
        Ok(data)
    }

    pub fn get_ship_weapons(&self, api: &mut P3) -> Result<Vec<u32>, P3ApiError> {
        let wares_count = ShipWeaponId::Cannon as usize + 1;
        let bytes_len = wares_count * mem::size_of::<u32>();
        let mut input_data: Vec<u8> = vec![0; bytes_len];
        api.read_memory(self.address + 0x0124, &mut input_data)?;
        let mut data: Vec<u32> = Vec::with_capacity(wares_count);
        for i in 0..wares_count {
            data.push(u32::from_le_bytes(input_data[i * 4..(i * 4) + 4].try_into().unwrap()))
        }
        Ok(data)
    }

    pub fn get_cutlasses(&self, api: &mut P3) -> Result<u32, P3ApiError> {
        self.get(0x2bc, api)
    }
}

impl<P3: P3AccessApi> P3Pointer for StoragePtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
