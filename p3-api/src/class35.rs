use std::{mem, ops::Deref, ptr};

use log::warn;

use crate::{
    data::{
        enums::{ShipType, TownId},
        p3_ptr::P3Pointer,
    },
    town::static_town_data::StaticTownDataPtr,
    Point,
};

const CLASS35_PTR_ADDRESS: *const u32 = 0x006CBDC8 as _;
pub const CAPACITY_FACTOR_MAX: u32 = 4096;
pub const HEALTH_FACTOR_MAX: u32 = 256;

#[derive(Clone, Debug, Copy)]
pub struct Class35Ptr {
    pub address: u32,
}

#[derive(Clone, Debug)]
pub struct ShipRoutePtr {
    pub address: u32,
    pub needs_free: bool,
}

impl ShipRoutePtr {
    pub unsafe fn calculate_distance(&self) -> i32 {
        let mut distance = 0;
        if self.len > 1 {
            for i in 1..self.len as usize {
                let p0 = (*self.points.add(i - 1)).clone();
                let p1 = (*self.points.add(i)).clone();
                let x_diff = (p0.x - p1.x).abs();
                let y_diff = (p0.y - p1.y).abs();
                if x_diff <= y_diff {
                    distance += (16 * y_diff + 7 * x_diff) as i32;
                } else {
                    distance += (16 * x_diff + 7 * y_diff) as i32;
                }
            }
        }

        distance
    }

    pub unsafe fn calculate_travel_time(&self, ship_type: ShipType, health_factor: u32, capacity_factor: u32) -> u32 {
        let capacity_scaled_speed = (ship_type.get_base_speed() as u32 * capacity_factor) >> 12;
        let speed_factor = (capacity_scaled_speed * health_factor) >> 10;
        8 * self.calculate_distance() as u32 / speed_factor
    }

    pub unsafe fn free(self) {
        if !self.needs_free {
            return;
        }
        if !self.points.is_null() {
            crate::free(self.points as usize as _);
        }
        crate::free(self.address);
    }
}

impl Deref for ShipRoutePtr {
    type Target = ShipRoute;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.address as *const ShipRoute) }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipRoute {
    pub points: *mut Point<i16>,
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

    pub unsafe fn get_nav_vec_entry(&self, offset: usize) -> (i16, i16) {
        let ptr: *const i16 = self.get(0x0c);
        (*ptr.add(2 * offset), *ptr.add(2 * offset + 1))
    }

    pub unsafe fn get_nav_vec_count(&self) -> i16 {
        self.get(0x1c)
    }

    pub unsafe fn calculate_town_route(&self, source: TownId, destination: TownId) -> Option<ShipRoutePtr> {
        let source_data = StaticTownDataPtr::new(source);
        let destination_data = StaticTownDataPtr::new(destination);
        self.calculate_route(&source_data.get_point_i16(), &destination_data.get_point_i16())
    }

    pub unsafe fn calculate_route(&self, source: &Point<i16>, destination: &Point<i16>) -> Option<ShipRoutePtr> {
        let args_inner = ShipRouteArgsInner {
            source: source.clone(),
            destination: destination.clone(),
        };
        let args = ShipRouteArgs {
            args_inner: &args_inner,
            route_type: 2,
        };
        let mut ship_route: *mut ShipRoute = ptr::null_mut();
        let orig: extern "thiscall" fn(this: u32, route_args: *const ShipRouteArgs, route: *mut *mut ShipRoute) -> bool = mem::transmute(0x00445010);
        let needs_free = orig(self.address, &args, &mut ship_route);
        if !ship_route.is_null() {
            Some(ShipRoutePtr {
                address: ship_route as _,
                needs_free,
            })
        } else {
            warn!("Failed to calculate route {:x}", ship_route as u32);
            None
        }
    }
}

impl P3Pointer for Class35Ptr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
