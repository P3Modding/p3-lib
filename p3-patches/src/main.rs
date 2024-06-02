#![no_std]
#![no_main]

use core::{marker::PhantomData, ptr::read_volatile};


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

const STATIC_CLASS35_PTR: u32 = 0x006CBDC8;

struct Class35Ptr {}

struct Class37Ptr {
    address: u32,
}

struct PointPtr<T> {
    address: u32,
    phantom: PhantomData<T>,
}

impl Class35Ptr {}

impl Class37Ptr {
    unsafe fn get_point(&self) -> PointPtr<i32> {
        PointPtr {
            address: self.address + 0x14,
            phantom: PhantomData,
        }
    }

    unsafe fn get_offset_thing(&self) -> PointPtr<i16> {
        PointPtr {
            address: self.address + 0x29e4,
            phantom: PhantomData,
        }
    }
}

impl<T> PointPtr<T> {
    unsafe fn get_x(&self) -> T {
        let ptr: *mut T = core::mem::transmute(self.address);
        ptr.read_volatile()
    }

    unsafe fn get_y(&self) -> i32 {
        let ptr: *mut i32 = core::mem::transmute(self.address + core::mem::size_of::<T>() as u32);
        ptr.read_volatile()
    }
}

#[no_mangle]
unsafe extern "thiscall" fn _start(this: u32, a2: u32) -> u32 {
    let class37 = Class37Ptr { address: this };
    let draw_geometry: extern "cdecl" fn(x1: i32, y1: i32, x1: i32, y2: i32) = core::mem::transmute(0x004BD680);
    let x = class37.get_offset_thing().get_x();
    let y = class37.get_offset_thing().get_y();
    draw_geometry(0-x as i32, 0-y as i32, 1000-x as i32, 1000-y as i32);
    let draw_navigation_line: extern "thiscall" fn(this: u32, a2: u32) -> u32 = core::mem::transmute(0x0044B330);
    draw_navigation_line(this, a2)
}
