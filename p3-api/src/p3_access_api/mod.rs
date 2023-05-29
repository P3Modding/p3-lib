pub mod local_p3_access_api;

pub trait P3AccessApi {
    fn read_memory(&mut self, address: u32, dst: &mut [u8]);
    fn read_u32(&mut self, address: u32) -> u32 {
        let mut buf = [0; 4];
        self.read_memory(address, &mut buf);
        u32::from_le_bytes(buf)
    }
}
