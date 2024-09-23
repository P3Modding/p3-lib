#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipCost {
    pub timber: u32,
    pub cloth: u32,
    pub iron_goods: u32,
    pub hemp: u32,
    pub pitch: u32,
    pub field_14: u32, // base_time?
    pub base_price: u32,
    pub field_1c: u32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipCosts {
    pub snaikka: [ShipCost; 3],
    pub craier: [ShipCost; 3],
    pub cog: [ShipCost; 3],
    pub hulk: [ShipCost; 3],
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipCapacityRaw {
    pub snaikka: [u8; 4],
    pub craier: [u8; 4],
    pub cog: [u8; 4],
    pub hulk: [u8; 4],
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ShipyardLevelRequirements {
    pub snaikka: [u16; 4],
    pub crayer: [u16; 4],
    pub cog: [u16; 4],
    pub holk: [u16; 4],
}

pub fn get_ship_costs() -> ShipCosts {
    let ptr: *const ShipCosts = 0x0066DEB0 as _;
    unsafe { (*ptr).clone() }
}

pub fn get_ship_capacities_raw() -> ShipCapacityRaw {
    let ptr: *const ShipCapacityRaw = 0x00673838 as _;
    unsafe { (*ptr).clone() }
}

pub fn get_shipyard_level_requirements() -> &'static ShipyardLevelRequirements {
    let ptr: *const ShipyardLevelRequirements = 0x00673818 as _;
    unsafe { &*ptr }
}
