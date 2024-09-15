use super::P3AccessApi;
use crate::P3ApiError;
use std::ptr;

#[derive(Debug)]
pub struct RawP3AccessApi {}

impl Default for RawP3AccessApi {
    fn default() -> Self {
        Self::new()
    }
}

impl RawP3AccessApi {
    pub const fn new() -> Self {
        Self {}
    }
}

impl P3AccessApi for RawP3AccessApi {
    fn read_memory(&self, address: u32, dst: &mut [u8]) -> Result<(), P3ApiError> {
        unsafe {
            debug_assert!(!dst.is_empty());
            ptr::copy_nonoverlapping(address as *const u8, dst.as_mut_ptr(), dst.len());
            Ok(())
        }
    }
}
