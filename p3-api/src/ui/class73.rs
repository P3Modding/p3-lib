use crate::data::{p3_ptr::P3Pointer, screen_game_ini_anim::ScreenGameIniAnimPtr};

pub const CLASS73_PTR_ADDRESS: *const u32 = 0x006CBA64 as _;

#[derive(Clone, Debug)]
pub struct Class73Ptr {
    pub address: u32,
}

impl Default for Class73Ptr {
    fn default() -> Self {
        Self::new()
    }
}

impl Class73Ptr {
    pub fn new() -> Self {
        Self {
            address: unsafe { *CLASS73_PTR_ADDRESS },
        }
    }

    pub unsafe fn set_x(&self, x: u32) {
        // TODO write through P3
        let ptr: *mut u32 = (self.address + 0x14) as _;
        *ptr = x;
    }

    pub unsafe fn set_y(&self, y: u32) {
        // TODO write through P3
        let ptr: *mut u32 = (self.address + 0x18) as _;
        *ptr = y;
    }

    pub unsafe fn set_width(&self, width: u32) {
        // TODO write through P3
        let ptr: *mut u32 = (self.address + 0x2c) as _;
        *ptr = width;
    }

    pub unsafe fn set_height(&self, height: u32) {
        // TODO write through P3
        let ptr: *mut u32 = (self.address + 0x30) as _;
        *ptr = height;
    }

    pub fn get_anim_0_1_2(&self) -> ScreenGameIniAnimPtr {
        ScreenGameIniAnimPtr::new(self.address + 0x164)
    }
}

impl P3Pointer for Class73Ptr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
