use crate::P3ApiError;

pub mod local_p3_access_api;

pub trait P3AccessApi {
    fn read_memory(&mut self, address: u32, dst: &mut [u8]) -> Result<(), P3ApiError>;
    fn read_u32(&mut self, address: u32) -> Result<u32, P3ApiError> {
        let mut buf = [0; 4];
        self.read_memory(address, &mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
}
