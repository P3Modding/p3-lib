use super::{p3_ptr::P3Pointer, storage::StoragePtr};

pub const OFFICE_SIZE: u32 = 0x44C;

#[derive(Debug)]
pub struct OfficePtr {
    pub address: u32,
}

impl OfficePtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_storage(&self) -> StoragePtr {
        StoragePtr::new(self.address)
    }

    pub fn get_merchant_id(&self) -> u16 {
        unsafe { self.get(0x2c4) }
    }

    pub fn next_office_id(&self) -> u16 {
        unsafe { self.get(0x2ca) }
    }
}

impl P3Pointer for OfficePtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
