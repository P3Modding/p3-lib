use super::{p3_ptr::P3Pointer, storage::StoragePtr};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::marker::PhantomData;

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

    pub fn get_storage(&self) -> StoragePtr<P3> {
        StoragePtr::new(self.address)
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
