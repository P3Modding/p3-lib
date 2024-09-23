use super::{enums::WareId, p3_ptr::P3Pointer};

#[derive(Debug)]
pub struct StoragePtr {
    pub address: u32,
}

impl StoragePtr {
    pub fn new(address: u32) -> Self {
        Self { address }
    }

    pub fn get_ware(&self, ware: WareId) -> i32 {
        unsafe { self.get(0x04 + ware as u32 * 4) }
    }

    pub fn get_wares(&self) -> [i32; 24] {
        unsafe { self.get(0x04) }
    }

    pub fn get_daily_consumptions_businesses(&self) -> [i32; 0x18] {
        unsafe { self.get(0x64) }
    }

    pub fn get_daily_production(&self) -> [i32; 0x18] {
        unsafe { self.get(0xc4) }
    }

    pub fn get_weird_daily_production(&self) -> [i32; 0x18] {
        unsafe { self.get(0x490) }
    }

    pub fn get_ship_weapons(&self) -> Vec<u32> {
        todo!()
        /*
        Removed during refactoring. If you need this, reach out
        let wares_count = ShipWeaponId::Cannon as usize + 1;
        let bytes_len = wares_count * mem::size_of::<u32>();
        let mut input_data: Vec<u8> = vec![0; bytes_len];
        api.read_memory(self.address + 0x0124, &mut input_data)?;
        let mut data: Vec<u32> = Vec::with_capacity(wares_count);
        for i in 0..wares_count {
            data.push(u32::from_le_bytes(input_data[i * 4..(i * 4) + 4].try_into().unwrap()))
        }
        Ok(data)*/
    }

    pub fn get_cutlasses(&self) -> u32 {
        unsafe { self.get(0x2bc) }
    }
}

impl P3Pointer for StoragePtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
