use super::{p3_ptr::P3Pointer, screen_game_ini_anim::ScreenGameIniAnimPtr};

pub const CLASS27_PTR_ADDRESS: *const u32 = 0x006CBB40 as _;

#[derive(Clone, Debug)]
pub struct Class27Ptr {
    pub address: u32,
}

impl Default for Class27Ptr {
    fn default() -> Self {
        Self::new()
    }
}

impl Class27Ptr {
    pub fn new() -> Self {
        Self {
            address: unsafe { *CLASS27_PTR_ADDRESS },
        }
    }

    pub fn get_anim_42(&self) -> ScreenGameIniAnimPtr {
        ScreenGameIniAnimPtr::new(self.address + 0xa90)
    }

    pub fn get_anim_44(&self) -> ScreenGameIniAnimPtr {
        ScreenGameIniAnimPtr::new(self.address + 0xdf0)
    }
}

impl P3Pointer for Class27Ptr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
