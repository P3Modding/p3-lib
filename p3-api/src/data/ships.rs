use super::{
    convoy::{ConvoyPtr, CONVOY_SIZE},
    p3_ptr::P3Pointer,
    ship::{ShipPtr, SHIP_SIZE},
};

pub const CLASS6_ADDRESS: u32 = 0x006dd7a0;

#[derive(Clone, Debug)]
pub struct ShipsPtr {
    pub address: u32,
}

impl Default for ShipsPtr {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipsPtr {
    pub const fn new() -> Self {
        Self { address: CLASS6_ADDRESS }
    }

    pub fn get_ship(&self, ship_id: u16) -> Option<ShipPtr> {
        if ship_id < self.get_ships_size() {
            let base_address: u32 = unsafe { self.get(0x04) };
            Some(ShipPtr::new(base_address + ship_id as u32 * SHIP_SIZE))
        } else {
            None
        }
    }

    pub fn get_ship_by_name(&self, name: &str) -> Option<(ShipPtr, u16)> {
        for i in 0..self.get_ships_size() {
            let ship = self.get_ship(i).unwrap();
            if name == ship.get_name() {
                return Some((ship, i));
            }
        }
        None
    }

    pub fn get_convoy(&self, convoy_id: u16) -> Option<ConvoyPtr> {
        if convoy_id < self.get_convoys_size() {
            let base_address: u32 = unsafe { self.get(0x08) };
            Some(ConvoyPtr::new(base_address + convoy_id as u32 * CONVOY_SIZE))
        } else {
            None
        }
    }

    pub fn get_ships_size(&self) -> u16 {
        unsafe { self.get(0xf4) }
    }

    pub fn get_convoys_size(&self) -> u16 {
        unsafe { self.get(0xf6) }
    }
}

impl P3Pointer for ShipsPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
