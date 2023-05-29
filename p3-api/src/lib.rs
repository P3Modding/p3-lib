use std::mem;

use log::debug;
use p3_access_api::P3AccessApi;
use structs::ship::{RawShip, Ship};

pub mod p3_access_api;
pub mod structs;

const SHIPS_PTR: u32 = 0x006DD7A4;

pub fn read_ship<Api: P3AccessApi>(api: &mut Api, ship_id: u16) -> Ship {
    let raw_ship = read_raw_ship(api, ship_id);
    Ship {
        merchant_id: raw_ship.field_0_merchant_id,
        max_health: raw_ship.field_14_max_health,
        current_health: raw_ship.field_18_current_health,
        x: raw_ship.field_1c_x,
        y: raw_ship.field_20_y,
        name: "TODO".to_string(),
        current_town_id: raw_ship.field_39_current_town_id,
    }
}

pub fn read_raw_ship<Api: P3AccessApi>(api: &mut Api, ship_id: u16) -> RawShip {
    let ships: u32 = api.read_u32(SHIPS_PTR);
    debug!("ships at {:#x}", ships);
    let ship_address: u32 = ships + (ship_id as u32 * mem::size_of::<RawShip>() as u32);
    let mut buf: [u8; mem::size_of::<RawShip>()] = [0; mem::size_of::<RawShip>()];
    debug!("Reading from {:#x}", ship_address);
    api.read_memory(ship_address, &mut buf);
    unsafe { std::ptr::read(buf.as_ptr() as *const _) }
}
