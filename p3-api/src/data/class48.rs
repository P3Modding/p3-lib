use super::p3_ptr::P3Pointer;

pub const CLASS48_PTR_ADDRESS: u32 = 0x006CC7E0;

#[derive(Clone, Debug)]
pub struct Class48Ptr {
    pub address: u32,
}

impl Default for Class48Ptr {
    fn default() -> Self {
        Self::new()
    }
}

impl Class48Ptr {
    pub fn new() -> Self {
        let ptr: *const u32 = CLASS48_PTR_ADDRESS as _;
        Self { address: unsafe { *ptr } }
    }

    pub fn set_gradient_y(&self, value: u16) {
        let ptr: *mut u16 = (self.address + 0xec) as _;
        unsafe { *ptr = value }
    }

    pub fn set_ignore_below_gradient(&self, value: u8) {
        let ptr: *mut u8 = (self.address + 0xfc) as _;
        unsafe { *ptr = value }
    }
}

impl P3Pointer for Class48Ptr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
