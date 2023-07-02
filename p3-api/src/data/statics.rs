use super::p3_ptr::{self};
use crate::{p3_access_api::P3AccessApi, P3ApiError};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipCost {
    pub timber: u32,
    pub cloth: u32,
    pub iron_goods: u32,
    pub hemp: u32,
    pub pitch: u32,
    pub field_14: u32, // base_time?
    pub base_price: u32,
    pub field_1c: u32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipCosts {
    pub snaikka: [ShipCost; 3],
    pub craier: [ShipCost; 3],
    pub cog: [ShipCost; 3],
    pub hulk: [ShipCost; 3],
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipCapacityRaw {
    pub snaikka: [u8; 4],
    pub craier: [u8; 4],
    pub cog: [u8; 4],
    pub hulk: [u8; 4],
}

pub fn get_ship_costs<P3: P3AccessApi>(api: &mut P3) -> Result<ShipCosts, P3ApiError> {
    p3_ptr::get_from(0x0066DEB0, api)
}

pub fn get_ship_capacities_raw<P3: P3AccessApi>(api: &mut P3) -> Result<ShipCapacityRaw, P3ApiError> {
    p3_ptr::get_from(0x00673838, api)
}
