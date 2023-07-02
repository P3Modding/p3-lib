use crate::P3ApiError;
use std::fmt::Debug;

pub mod open_process_p3_access_api;
pub mod raw_p3_access_api;

pub trait P3AccessApi: Debug {
    fn read_memory(&self, address: u32, dst: &mut [u8]) -> Result<(), P3ApiError>;
    fn read_u32(&self, address: u32) -> Result<u32, P3ApiError> {
        let mut buf = [0; 4];
        self.read_memory(address, &mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    fn read_u16(&self, address: u32) -> Result<u16, P3ApiError> {
        let mut buf = [0; 2];
        self.read_memory(address, &mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
}
