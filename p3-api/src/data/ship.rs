use super::p3_ptr::P3Pointer;
use crate::{latin1_to_string, p3_access_api::P3AccessApi, P3ApiError};
use num_traits::FromPrimitive;
use std::marker::PhantomData;

pub const SHIP_SIZE: u32 = 0x180;

#[derive(Debug)]
pub struct ShipPtr<P3> {
    pub address: u32,
    api_type: PhantomData<P3>,
}

impl<P3: P3AccessApi> ShipPtr<P3> {
    pub fn new(address: u32) -> Self {
        Self {
            address,
            api_type: PhantomData,
        }
    }

    pub fn get_raw(&self) -> *mut RawShip {
        self.address as _
    }

    pub fn get_next_ship_in_convoy(&self, api: &P3) -> Result<u16, P3ApiError> {
        let ship_id = self.get(0x06, api)?;
        Ok(ship_id)
    }

    pub fn get_convoy_id(&self, api: &P3) -> Result<u16, P3ApiError> {
        let convoy_id = self.get(0x08, api)?;
        Ok(convoy_id)
    }

    pub fn get_capacity(&self, api: &P3) -> Result<u32, P3ApiError> {
        let capacity = self.get(0x10, api)?;
        Ok(capacity)
    }

    pub fn get_max_health(&self, api: &P3) -> Result<u32, P3ApiError> {
        let max_heatlh = self.get(0x14, api)?;
        Ok(max_heatlh)
    }

    pub fn get_current_health(&self, api: &P3) -> Result<u32, P3ApiError> {
        let current_health = self.get(0x18, api)?;
        Ok(current_health)
    }

    pub fn get_x(&self, api: &P3) -> Result<i32, P3ApiError> {
        self.get(0x1c, api)
    }

    pub fn get_y(&self, api: &P3) -> Result<i32, P3ApiError> {
        self.get(0x20, api)
    }

    pub fn get_destination_town_index(&self, api: &P3) -> Result<Option<u8>, P3ApiError> {
        let town_index: u8 = self.get(0x38, api)?;
        Ok(FromPrimitive::from_u8(town_index))
    }

    pub fn get_last_town_index(&self, api: &P3) -> Result<Option<u8>, P3ApiError> {
        let town_index: u8 = self.get(0x39, api)?;
        if town_index != 0xff {
            Ok(Some(town_index))
        } else {
            Ok(None)
        }
    }

    pub fn get_status(&self, api: &P3) -> Result<u16, P3ApiError> {
        let raw_status = self.get(0x134, api)?;
        Ok(raw_status)
    }

    pub fn get_name(&self, api: &P3) -> Result<String, P3ApiError> {
        let buf: [u8; 16] = self.get(0x160, api)?;
        Ok(latin1_to_string(&buf))
    }
}

impl<P3: P3AccessApi> P3Pointer for ShipPtr<P3> {
    fn get_address(&self) -> u32 {
        self.address
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct RawShip {
    pub field_0_merchant_id: u8,
    pub field_1: u8,
    pub field_2: u16,
    pub field_4: i32,
    pub field_8: u16,
    pub field_a_some_ship_id: u16,
    pub field_c: u16,
    pub field_e: u16,
    pub field_10: i32,
    pub field_14_max_health: i32,
    pub field_18_current_health: i32,
    pub field_1c_x: i32,
    pub field_20_y: i32,
    pub field_24: u32,
    pub field_28_x_delta: i32,
    pub field_2c_y_delta: i32,
    pub field_30: i32,
    pub field_34: i32,
    pub field_38: u8,
    pub field_39_current_town_id: u8,
    pub field_3a: u16,
    pub field_3c: i32,
    pub field_40: u16,
    pub field_42: u16,
    pub field_44: i32,
    pub field_48: i32,
    pub field_4c: i32,
    pub field_50: i32,
    pub field_54: i32,
    pub field_58: i32,
    pub field_5c: i32,
    pub field_60: i32,
    pub field_64: i32,
    pub field_68: i32,
    pub field_6c: i32,
    pub field_70: i32,
    pub field_74: i32,
    pub field_78: i32,
    pub field_7c: i32,
    pub field_80: i32,
    pub field_84: i32,
    pub field_88: i32,
    pub field_8c: i32,
    pub field_90: i32,
    pub field_94: i32,
    pub field_98: i32,
    pub field_9c: i32,
    pub field_a0: i32,
    pub field_a4: i32,
    pub field_a8: i32,
    pub field_ac: i32,
    pub field_b0: i32,
    pub field_b4: i32,
    pub field_b8: i32,
    pub field_bc: i32,
    pub field_c0: i32,
    pub field_c4: i32,
    pub field_c8: i32,
    pub field_cc: i32,
    pub field_d0: i32,
    pub field_d4: i32,
    pub field_d8: i32,
    pub field_dc: i32,
    pub field_e0: i32,
    pub field_e4: i32,
    pub field_e8: i32,
    pub field_ec: i32,
    pub field_f0: i32,
    pub field_f4: i32,
    pub field_f8: i32,
    pub field_fc: i32,
    pub field_100: i32,
    pub field_104: i32,
    pub field_108: i32,
    pub field_10c: i32,
    pub field_110: i32,
    pub field_114: i32,
    pub field_118: i32,
    pub field_11c: i32,
    pub field_120: i32,
    pub field_124: i32,
    pub field_128: i32,
    pub field_12c: i32,
    pub field_130: i32,
    pub field_134_flags: u16,
    pub field_136: u8,
    pub field_137: u8,
    pub field_138: u8,
    pub field_139: u8,
    pub field_13a: u8,
    pub field_13b: u8,
    pub field_13c: i32,
    pub field_140: i32,
    pub field_144: i32,
    pub field_148: i32,
    pub field_14c: i32,
    pub field_150: i32,
    pub field_154: i32,
    pub field_158: i32,
    pub field_15c: i32,
    pub field_160_ship_name: [u8; 16],
    pub field_170: i32,
    pub field_174: i32,
    pub field_178: i32,
    pub field_17c: i32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Ship {
    pub merchant_id: u8,
    pub max_health: i32,
    pub current_health: i32,
    pub x: i32,
    pub y: i32,
    pub name: String,
    pub current_town_id: u8,
}
