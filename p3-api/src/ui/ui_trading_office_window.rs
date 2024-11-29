use crate::data::p3_ptr::P3Pointer;

pub const STATIC_UI_TRADING_OFFICE_WINDOW_PTR_ADDRESS: *const u32 = 0x006E557C as _;

#[derive(Clone, Debug, Copy)]
pub struct UITradingOfficeWindowPtr {
    address: u32,
}

impl Default for UITradingOfficeWindowPtr {
    fn default() -> Self {
        Self::new()
    }
}

impl UITradingOfficeWindowPtr {
    pub const VTABLE_OFFSET: u32 = 0x279CB0;
    pub fn new() -> Self {
        Self {
            address: unsafe { *STATIC_UI_TRADING_OFFICE_WINDOW_PTR_ADDRESS },
        }
    }

    pub unsafe fn get_town_index(&self) -> u16 {
        self.get(0xecc8)
    }
}

impl P3Pointer for UITradingOfficeWindowPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
