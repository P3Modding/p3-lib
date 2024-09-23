use super::{
    p3_ptr::P3Pointer,
    screen_rectangle::{ScreenRectangleArrayPtr, ScreenRectanglePtr},
};

#[derive(Clone, Debug)]
pub struct ScreenGameIniAnimPtr {
    pub address: u32,
}

impl ScreenGameIniAnimPtr {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn set_pos_x(&self, x: u32) {
        // TODO write through P3
        let ptr: *mut u32 = (self.address + 0x20) as _;
        unsafe {
            *ptr = x;
        }
    }

    pub fn get_screen_rectangle_array_ptr(&self) -> ScreenRectangleArrayPtr {
        ScreenRectangleArrayPtr::new(unsafe { self.get(0xa0) })
    }

    pub fn get_screen_rectangle(&self, index: u32) -> ScreenRectanglePtr {
        self.get_screen_rectangle_array_ptr().get_screen_rectangle(index)
    }
}

impl P3Pointer for ScreenGameIniAnimPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
