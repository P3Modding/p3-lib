use super::{p3_ptr::P3Pointer, storage::StoragePtr};

pub const OFFICE_SIZE: u32 = 0x44C;

#[derive(Debug, Clone, Copy)]
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

    pub fn get_merchant_index(&self) -> u16 {
        unsafe { self.get(0x2c4) }
    }

    pub fn get_next_office_of_merchant_index(&self) -> u16 {
        unsafe { self.get(0x2c8) }
    }

    pub fn get_next_office_in_town_index(&self) -> u16 {
        unsafe { self.get(0x2ca) }
    }

    pub unsafe fn get_administrator_trade_prices(&self) -> [i32; 20] {
        self.get(0x2f4)
    }

    pub unsafe fn set_administrator_trade_prices(&self, prices: [i32; 20]) {
        self.set(0x2f4, &prices)
    }

    pub unsafe fn set_administrator_trade_actions(&self, actions: [i32; 20]) {
        self.set(0x2f4, &actions)
    }

    pub unsafe fn get_administrator_trade_stock(&self) -> [i32; 20] {
        self.get(0x354)
    }

    pub unsafe fn get_administrator_trade_lock_bitmap(&self) -> u32 {
        self.get(0x3b4)
    }
}

impl P3Pointer for OfficePtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
