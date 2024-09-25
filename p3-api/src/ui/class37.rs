use crate::data::p3_ptr::P3Pointer;

pub const CLASS37_PTR_ADDRESS: *const u32 = 0x006CBDD8 as _;

#[derive(Clone, Debug)]
pub struct Class37Ptr {
    pub address: u32,
}

impl Default for Class37Ptr {
    fn default() -> Self {
        Self::new()
    }
}

impl Class37Ptr {
    pub fn new() -> Self {
        Self {
            address: unsafe { *CLASS37_PTR_ADDRESS },
        }
    }

    pub fn get_x(&self) -> i16 {
        unsafe { self.get(0x29e4) }
    }

    pub fn get_y(&self) -> i16 {
        unsafe { self.get(0x29e6) }
    }
}

impl P3Pointer for Class37Ptr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
