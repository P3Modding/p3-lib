use crate::data::p3_ptr::P3Pointer;

pub const STATIC_UI_TOWN_HALL_WINDOW_PTR_ADDRESS: *const u32 = 0x006E558C as _;

#[derive(Clone, Debug)]
pub struct UITownHallWindowPtr {
    pub address: u32,
}

impl Default for UITownHallWindowPtr {
    fn default() -> Self {
        Self::new()
    }
}

impl UITownHallWindowPtr {
    pub const VTABLE_OFFSET: u32 = 0x27A050;
    pub fn new() -> Self {
        Self {
            address: unsafe { *STATIC_UI_TOWN_HALL_WINDOW_PTR_ADDRESS },
        }
    }

    pub fn get_x(&self) -> i32 {
        unsafe { self.get(0x14) }
    }

    pub fn get_y(&self) -> i32 {
        unsafe { self.get(0x18) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { self.get(0x2c) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { self.get(0x30) }
    }

    pub fn get_selected_page(&self) -> i32 {
        unsafe { self.get(0x18e4) }
    }

    pub unsafe fn get_selected_alderman_mission_index(&self) -> u8 {
        self.get(0x1907)
    }

    pub unsafe fn get_task_index(&self, mission_index: u8) -> u16 {
        self.get(0x1946 + mission_index as u32 * 2)
    }

    pub unsafe fn get_next_mission_index(&self) -> u8 {
        self.get(0x1966)
    }
}

impl P3Pointer for UITownHallWindowPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
