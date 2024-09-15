use super::{
    convoy::{ConvoyPtr, CONVOY_SIZE},
    p3_ptr::P3Pointer,
    ship::{ShipPtr, SHIP_SIZE},
};
use crate::{p3_access_api::P3AccessApi, P3ApiError};
use std::marker::PhantomData;

pub const CLASS6_ADDRESS: u32 = 0x006dd7a0;

#[derive(Clone, Debug)]
pub struct ShipsPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> ShipsPtr<P3> {
    pub const fn new() -> Self {
        Self {
            address: CLASS6_ADDRESS,
            api_type: PhantomData,
        }
    }

    pub fn get_ship(&self, ship_id: u16, api: &P3) -> Result<Option<ShipPtr<P3>>, P3ApiError> {
        if ship_id < self.get_ships_size(api)? {
            let base_address: u32 = self.get(0x04, api)?;
            Ok(Some(ShipPtr::new(base_address + ship_id as u32 * SHIP_SIZE)))
        } else {
            Ok(None)
        }
    }

    pub fn get_ship_by_name(&self, name: &str, api: &P3) -> Result<Option<(ShipPtr<P3>, u16)>, P3ApiError> {
        for i in 0..self.get_ships_size(api).unwrap() {
            let ship = self.get_ship(i, api).unwrap().unwrap();
            if name == ship.get_name(api)? {
                return Ok(Some((ship, i)));
            }
        }
        Ok(None)
    }

    pub fn get_convoy(&self, convoy_id: u16, api: &P3) -> Result<Option<ConvoyPtr<P3>>, P3ApiError> {
        if convoy_id < self.get_convoys_size(api)? {
            let base_address: u32 = self.get(0x08, api)?;
            Ok(Some(ConvoyPtr::new(base_address + convoy_id as u32 * CONVOY_SIZE)))
        } else {
            Ok(None)
        }
    }

    pub fn get_ships_size(&self, api: &P3) -> Result<u16, P3ApiError> {
        self.get(0xf4, api)
    }

    pub fn get_convoys_size(&self, api: &P3) -> Result<u16, P3ApiError> {
        self.get(0xf6, api)
    }
}

impl<P3: P3AccessApi> P3Pointer for ShipsPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}
