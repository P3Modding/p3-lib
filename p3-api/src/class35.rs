use std::{ffi::c_void, mem, ptr};

use log::debug;

use crate::{data::p3_ptr::P3Pointer, Point};

const CLASS35_PTR_ADDRESS: *const u32 = 0x006CBDC8 as _;

#[derive(Clone, Debug)]
pub struct Class35Ptr {
    pub address: u32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipRoute {
    pub points: *const c_void,
    pub len: i32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipRouteArgs {
    pub args_inner: *const ShipRouteArgsInner,
    pub route_type: i32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipRouteArgsInner {
    pub source: Point<i16>,
    pub destination: Point<i16>,
}

impl Class35Ptr {
    pub unsafe fn new() -> Self {
        Self { address: *CLASS35_PTR_ADDRESS }
    }

    pub unsafe fn calculate_ship_route(&self, source: Point<i16>, destination: Point<i16>) -> *const ShipRoute {
        let args_inner = ShipRouteArgsInner { source, destination };
        let args = ShipRouteArgs {
            args_inner: &args_inner,
            route_type: 2,
        };
        let mut ship_route: *const ShipRoute = ptr::null_mut();
        debug!("Calling calculate_ship_route orig");
        let orig: extern "thiscall" fn(this: u32, route_args: *const ShipRouteArgs, route: *mut *const ShipRoute) -> i32 = mem::transmute(0x00445010);
        orig(self.address, &args, &mut ship_route);
        debug!("Calling calculate_ship_route survived! {ship_route:?}");
        ship_route
    }
}

impl P3Pointer for Class35Ptr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
