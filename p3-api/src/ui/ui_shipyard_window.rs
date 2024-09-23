use crate::data::p3_ptr::P3Pointer;

pub const STATIC_UI_SHIPYARD_WINDOW_PTR_ADDRESS: *const u32 = 0x006E55C0 as _;

#[derive(Clone, Debug)]
pub struct UIShipyardWindowPtr {
    pub address: u32,
}

impl Default for UIShipyardWindowPtr {
    fn default() -> Self {
        Self::new()
    }
}

impl UIShipyardWindowPtr {
    pub fn new() -> Self {
        Self {
            address: unsafe { *STATIC_UI_SHIPYARD_WINDOW_PTR_ADDRESS },
        }
    }

    pub fn get_x(&self) -> i32 {
        unsafe { self.get(0x14) }
    }

    pub fn get_y(&self) -> i32 {
        unsafe { self.get(0x18) }
    }

    pub fn get_selected_page(&self) -> i32 {
        unsafe { self.get(0xc7c) }
    }

    pub fn get_town_index(&self) -> i32 {
        unsafe { self.get(0xcb8) }
    }
}

impl P3Pointer for UIShipyardWindowPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
