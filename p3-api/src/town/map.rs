use crate::data::p3_ptr::P3Pointer;

#[derive(Debug)]
pub struct TownMapPtr {
    pub address: u32,
}

impl TownMapPtr {
    pub fn get_rows(&self) -> u32 {
        unsafe { self.get(0x08) }
    }

    pub fn get_cols(&self) -> u32 {
        unsafe { self.get(0x04) }
    }

    pub fn get_map_data(&self) -> Vec<u8> {
        /*
        not yet fixed after refactoring work, if you need this please reach out
        let address: u32 = self.get(0x68);
        let cols = self.get_cols(api)? as usize;
        let rows = self.get_rows(api)? as usize;
        let mut buf: Vec<u8> = vec![0; cols * rows];
        api.read_memory(address, &mut buf)?;
        Ok(buf)
        */
        todo!()
    }
}

impl P3Pointer for TownMapPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
