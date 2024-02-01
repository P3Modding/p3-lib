use crate::ffi;

#[repr(C)]
#[derive(Debug)]
pub struct RawAimFile {
    pub buf_ptr: *const u8,
    pub palette_ptr: *const u32,
    pub width1: u32,
    pub height: u32,
    pub pixel_encoding: u32,
    pub bytes_per_pixel2: u32,
    pub width2: u32,
    pub field_1c: u32,
    pub field_20: u32,
    pub field_24: u32,
    pub field_28: u32,
    pub field_2c: u32,
    pub field_30: u32,
    pub field_34: u32,
    pub field_38: u32,
    pub field_3c: u32,
    pub field_40: u32,
    pub field_44: u32,
    pub field_48: u32,
    pub field_4c: u32,
    pub field_50: u32,
    pub field_54: u32,
    pub inner_ptr: u32,
}

impl Default for RawAimFile {
    fn default() -> Self {
        Self {
            buf_ptr: 0 as _,
            palette_ptr: 0 as _,
            width1: Default::default(),
            height: Default::default(),
            pixel_encoding: Default::default(),
            bytes_per_pixel2: Default::default(),
            width2: Default::default(),
            field_1c: Default::default(),
            field_20: Default::default(),
            field_24: Default::default(),
            field_28: Default::default(),
            field_2c: Default::default(),
            field_30: Default::default(),
            field_34: Default::default(),
            field_38: Default::default(),
            field_3c: Default::default(),
            field_40: Default::default(),
            field_44: Default::default(),
            field_48: Default::default(),
            field_4c: Default::default(),
            field_50: Default::default(),
            field_54: Default::default(),
            inner_ptr: Default::default(),
        }
    }
}

impl Drop for RawAimFile {
    fn drop(&mut self) {
        unsafe {
            if self.inner_ptr != 0 {
                ffi::aim_free(self);
            }
        }
    }
}
