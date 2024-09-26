use super::p3_ptr::P3Pointer;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ScreenRectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Clone, Debug)]
pub struct ScreenRectanglePtr {
    pub address: u32,
}

impl ScreenRectanglePtr {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn set_width(&self, width: u32) {
        // TODO write through P3
        let ptr: *mut u32 = (self.address + 0x08) as _;
        unsafe {
            *ptr = width;
        }
    }

    pub fn set_height(&self, height: u32) {
        // TODO write through P3
        let ptr: *mut u32 = (self.address + 0x0c) as _;
        unsafe {
            *ptr = height;
        }
    }
}

impl P3Pointer for ScreenRectanglePtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

#[derive(Clone, Debug)]
pub struct ScreenRectangleArrayPtr {
    pub address: u32,
}

impl ScreenRectangleArrayPtr {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_screen_rectangle(&self, index: u32) -> ScreenRectanglePtr {
        //TODO SECURITY enforce size
        ScreenRectanglePtr::new(unsafe { self.get(index * 4) })
    }
}

impl P3Pointer for ScreenRectangleArrayPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
