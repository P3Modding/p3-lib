#![allow(clippy::missing_safety_doc)]
use crate::{
    data::enums::{TownId, WareId},
    export::{OfficeData, TownData},
    p3_access_api::open_process_p3_access_api::OpenProcessP3AccessApi,
};

use std::ptr;

/// Open Patrician3.exe for Reading.
#[no_mangle]
pub extern "C" fn new_api(pid: u32) -> *mut DotnetOpenProcessP3AccessApi {
    match OpenProcessP3AccessApi::new(pid) {
        Ok(api) => Box::into_raw(Box::new(api)) as _,
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn read_town(api: *mut DotnetOpenProcessP3AccessApi, raw_town_id: TownId) -> *const TownData {
    unsafe {
        let api = api as *mut OpenProcessP3AccessApi;
        let api: &mut OpenProcessP3AccessApi = &mut *api;
        let town = match TownData::read(raw_town_id, api) {
            Ok(town) => town,
            Err(_) => return ptr::null(),
        };
        match town {
            Some(town) => Box::into_raw(Box::new(town)),
            None => ptr::null(),
        }
    }
}

#[no_mangle]
pub extern "C" fn get_ware_scaling(ware_id: WareId) -> i32 {
    ware_id.get_scaling()
}

#[no_mangle]
pub unsafe extern "C" fn free_town(town: *mut TownData) {
    drop(Box::from_raw(town))
}

#[no_mangle]
pub extern "C" fn dummy_office_data(_town: *mut OfficeData) {}

#[repr(C)]
pub struct DotnetOpenProcessP3AccessApi;
