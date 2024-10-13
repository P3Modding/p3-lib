use crate::data::p3_ptr::P3Pointer;

pub const MERCHANT_SIZE: u32 = 0x650;

#[derive(Clone, Debug)]
pub struct MerchantPtr {
    pub address: u32,
}

impl MerchantPtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_first_ship_index(&self) -> u16 {
        unsafe { self.get(0x0e) }
    }
}

impl P3Pointer for MerchantPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
