use crate::{p3_access_api::P3AccessApi, P3ApiError};

use super::p3_ptr::{self};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipCost {
    timber: u32,
    cloth: u32,
    iron_goods: u32,
    hemp: u32,
    pitch: u32,
    field_14: u32, // base_time?
    base_price: u32,
    field_1c: u32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipCosts {
    pub snaikka: [ShipCost; 3],
    pub craier: [ShipCost; 3],
    pub cog: [ShipCost; 3],
    pub hulk: [ShipCost; 3],
}

pub fn get_ship_costs<P3: P3AccessApi>(api: &mut P3) -> Result<ShipCosts, P3ApiError> {
    p3_ptr::get_from(0x0066DEB0, api)
}
