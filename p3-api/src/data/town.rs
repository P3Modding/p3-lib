use super::{p3_ptr::P3Pointer, storage::StoragePtr};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::marker::PhantomData;

pub const TOWN_SIZE: u32 = 0x9F8;

#[derive(Debug)]
pub struct TownPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

#[derive(Debug)]
pub struct TownMapPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

#[derive(Debug)]
#[repr(C)]
pub struct ShipLevels {
    pub snaikka_level: u8,
    pub craier_level: u8,
    pub cog_level: u8,
    pub hulk_level: u8,
}

impl<P3: P3AccessApi> TownPtr<P3> {
    pub fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_storage(&self) -> StoragePtr<P3> {
        StoragePtr::new(self.address)
    }

    pub fn get_raw_town_id(&self, api: &P3) -> Result<u8, P3ApiError> {
        self.get(0x2c1, api)
    }

    pub fn get_daily_consumptions_citizens(&self, api: &P3) -> Result<[i32; 24], P3ApiError> {
        self.get(0x310, api)
    }

    pub fn get_first_office_id(&self, api: &P3) -> Result<u16, P3ApiError> {
        self.get(0x784, api)
    }

    pub fn get_town_map(&self) -> TownMapPtr<P3> {
        TownMapPtr {
            address: self.address + 0x7a4,
            api_type: PhantomData,
        }
    }

    pub fn get_build_ship_capacity_markup(&self, api: &P3) -> Result<f32, P3ApiError> {
        self.get(0x818, api)
    }

    pub fn get_build_ship_levels(&self, api: &P3) -> Result<ShipLevels, P3ApiError> {
        self.get(0x824, api)
    }

    pub fn get_build_ship_828_always_0(&self, api: &P3) -> Result<[u8; 4], P3ApiError> {
        self.get(0x828, api)
    }
}

impl<P3: P3AccessApi> TownMapPtr<P3> {
    pub fn get_rows(&self, api: &P3) -> Result<u32, P3ApiError> {
        self.get(0x08, api)
    }

    pub fn get_cols(&self, api: &P3) -> Result<u32, P3ApiError> {
        self.get(0x04, api)
    }

    pub fn get_map_data(&self, api: &P3) -> Result<Vec<u8>, P3ApiError> {
        let address: u32 = self.get(0x68, api)?;
        let cols = self.get_cols(api)? as usize;
        let rows = self.get_rows(api)? as usize;
        let mut buf: Vec<u8> = vec![0; cols * rows];
        api.read_memory(address, &mut buf)?;
        Ok(buf)
    }
}

impl<P3: P3AccessApi> P3Pointer for TownPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}

impl<P3: P3AccessApi> P3Pointer for TownMapPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
