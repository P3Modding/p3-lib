use crate::data::p3_ptr::P3Pointer;

pub const STATIC_UI_TOWN_HALL_SIDEMENU_PTR_ADDRESS: *const u32 = 0x006E5500 as _;

#[derive(Clone, Debug)]
pub struct UITownHallSidemenuPtr {
    pub address: u32,
}

impl Default for UITownHallSidemenuPtr {
    fn default() -> Self {
        Self::new()
    }
}

impl UITownHallSidemenuPtr {
    pub const fn new() -> Self {
        Self {
            address: unsafe { *STATIC_UI_TOWN_HALL_SIDEMENU_PTR_ADDRESS },
        }
    }

    pub unsafe fn set_window_needs_redraw(&self) {
        let ptr: *mut u8 = (self.address + 0x8f7) as _;
        *ptr = 1;
    }
}

impl P3Pointer for UITownHallSidemenuPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
