use data::ship::{RawShip, Ship};
use log::debug;
use p3_access_api::P3AccessApi;
use std::mem;

extern crate num_derive;

pub mod data;
pub mod dotnet;
pub mod export;
pub mod p3_access_api;
pub use strum;

const SHIPS_PTR: u32 = 0x006DD7A4;

#[derive(Clone, Debug)]
pub enum P3ApiError {
    ReadError,
}

pub fn read_ship<Api: P3AccessApi>(api: &Api, ship_id: u16) -> Result<Ship, P3ApiError> {
    let raw_ship = read_raw_ship(api, ship_id)?;
    Ok(Ship {
        merchant_id: raw_ship.field_0_merchant_id,
        max_health: raw_ship.field_14_max_health,
        current_health: raw_ship.field_18_current_health,
        x: raw_ship.field_1c_x,
        y: raw_ship.field_20_y,
        name: latin1_to_string(&raw_ship.field_160_ship_name),
        current_town_id: raw_ship.field_39_current_town_id,
    })
}

pub fn read_raw_ship<Api: P3AccessApi>(api: &Api, ship_id: u16) -> Result<RawShip, P3ApiError> {
    let ships: u32 = api.read_u32(SHIPS_PTR)?;
    debug!("Ships array is at {:#x}", ships);
    let ship_address: u32 = ships + (ship_id as u32 * mem::size_of::<RawShip>() as u32);
    let mut buf: [u8; mem::size_of::<RawShip>()] = [0; mem::size_of::<RawShip>()];
    debug!("Reading from {:#x}", ship_address);
    api.read_memory(ship_address, &mut buf)?;
    Ok(unsafe { std::ptr::read(buf.as_ptr() as *const _) })
}

// https://stackoverflow.com/a/28175593/1569755
fn latin1_to_string(s: &[u8]) -> String {
    s.iter().take_while(|c| **c != 0).map(|&c| c as char).collect()
}
