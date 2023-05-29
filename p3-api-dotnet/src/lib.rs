use std::ptr;

use p3_api::p3_access_api::open_process_p3_access_api::OpenProcessP3AccessApi;
use structs::ship::DotnetShip;

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
/// `api` must point toward a valid OpenProcessP3AccessApi.
pub unsafe extern "C" fn read_ship(
    api: *mut DotnetOpenProcessP3AccessApi,
    ship_id: u16,
) -> *const DotnetShip {
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

#[repr(C)]
pub struct DotnetOpenProcessP3AccessApi;
