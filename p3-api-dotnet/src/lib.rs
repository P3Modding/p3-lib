use std::ptr;
use num_traits::cast::FromPrimitive;
use p3_api::{p3_access_api::open_process_p3_access_api::OpenProcessP3AccessApi, data::{enums::TownId, game_world::GameWorldPtr}};
use structs::{ship::DotnetShip, town::DotnetTown};

pub mod structs;

/// Open Patrician3.exe for Reading.
#[no_mangle]
pub extern "C" fn new_api(pid: u32) -> *mut DotnetOpenProcessP3AccessApi {
    match OpenProcessP3AccessApi::new(pid) {
        Ok(api) => Box::into_raw(Box::new(api)) as _,
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
/// Reads a ship from memory.
///
/// # Safety
///
/// `api` must point toward a valid DotnetOpenProcessP3AccessApi.
pub extern "C" fn read_ship(api: *mut DotnetOpenProcessP3AccessApi, ship_id: u16) -> *const DotnetShip {
    unsafe {
        // TODO wrap in lock?
        let api = api as *mut OpenProcessP3AccessApi;
        let api: &mut OpenProcessP3AccessApi = &mut *api;
        match p3_api::read_ship(api, ship_id) {
            Ok(ship) => Box::into_raw(Box::new(DotnetShip::from(ship))),
            Err(_) => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern "C" fn read_town(api: *mut DotnetOpenProcessP3AccessApi, town_id: u16) -> *const DotnetTown {
    unsafe {
        let api = api as *mut OpenProcessP3AccessApi;
        let api: &mut OpenProcessP3AccessApi = &mut *api;
        let game_word = GameWorldPtr::new();
        match DotnetTown::from_ptr(game_word.get_town(TownId::from_u16(town_id).unwrap(), api).unwrap(), api) {
            Ok(town) => Box::into_raw(Box::new(town)),
            Err(_) => ptr::null(),
        }
    }
}

#[repr(C)]
pub struct DotnetOpenProcessP3AccessApi;
