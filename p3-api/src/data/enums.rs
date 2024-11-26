use num_derive::FromPrimitive;
use strum::{EnumIter, EnumString};

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, EnumString, FromPrimitive)]
#[repr(u16)]
pub enum WareId {
    Grain = 0x00,
    Meat = 0x01,
    Fish = 0x02,
    Beer = 0x03,
    Salt = 0x04,
    Honey = 0x05,
    Spices = 0x06,
    Wine = 0x07,
    Cloth = 0x08,
    Skins = 0x09,
    WhaleOil = 0x0a,
    Timber = 0x0b,
    IronGoods = 0x0c,
    Leather = 0x0d,
    Wool = 0x0e,
    Pitch = 0x0f,
    PigIron = 0x10,
    Hemp = 0x11,
    Pottery = 0x12,
    Bricks = 0x13,
    Sword = 0x14,
    Bow = 0x15,
    Crossbow = 0x16,
    Carbine = 0x17,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, EnumString, FromPrimitive)]
#[repr(u16)]
pub enum FacilityId {
    Militia = 0x00,
    Shipyard = 0x01,
    Construction = 0x02,
    Weaponsmith = 0x03,
    HuntingLodge = 0x04,
    FishermansHouse = 0x05,
    Brewery = 0x06,
    Workshop = 0x07,
    Apiary = 0x08,
    GrainFarm = 0x09,
    CattleFarm = 0x0a,
    Sawmill = 0x0b,
    WeavingMill = 0x0c,
    Saltery = 0x0d,
    Ironsmelter = 0x0e,
    SheepFarm = 0x0f,
    Vineyard = 0x10,
    Pottery = 0x11,
    Brickworks = 0x12,
    Pitchmaker = 0x13,
    HempFarm = 0x14,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, EnumString, FromPrimitive)]
#[repr(u8)]
pub enum TownId {
    Edinburgh = 0x00,
    Newcastle = 0x01,
    Scarborough = 0x02,
    Boston = 0x03,
    London = 0x04,
    Bruges = 0x05,
    Haarlem = 0x06,
    Harlingen = 0x07,
    Groningen = 0x08,
    Cologne = 0x09,
    Bremen = 0x0a,
    Ripen = 0x0b,
    Hamburg = 0x0c,
    Flensburg = 0x0d,
    Luebeck = 0x0e,
    Rostock = 0x0f,
    Bergen = 0x10,
    Stavanger = 0x11,
    Toensberg = 0x12,
    Oslo = 0x13,
    Aalborg = 0x14,
    Goeteborg = 0x15,
    Naestved = 0x16,
    Malmoe = 0x17,
    Ahus = 0x18,
    Stockholm = 0x19,
    Visby = 0x1a,
    Helsinki = 0x1b,
    Stettin = 0x1c,
    Ruegenwald = 0x1d,
    Gdansk = 0x1e,
    Torun = 0x1f,
    Koenigsberg = 0x20,
    Memel = 0x21,
    Windau = 0x22,
    Riga = 0x23,
    Pernau = 0x24,
    Reval = 0x25,
    Ladoga = 0x26,
    Novgorod = 0x27,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, FromPrimitive)]
pub enum ShipWeaponId {
    SmallCatapult = 0x00,
    SmallBallista = 0x01,
    LargeCatapult = 0x02,
    LargeBallista = 0x03,
    Bombard = 0x04,
    Cannon = 0x05,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, FromPrimitive)]
#[repr(u8)]
pub enum ShipType {
    Snaikkka = 0x00,
    Craier = 0x01,
    Cog = 0x02,
    Hulk = 0x03,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, FromPrimitive)]
pub enum BuildingId {
    // Road = 0x24, uses opcode 0x25
    Well = 0x28,
    HousePoor = 0x2a,
    Fisher = 0x57,
    Apiary = 0x59,
    Warehouse = 0x66, // has a 0x20 next to it
    Mint = 0x67,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, FromPrimitive)]
#[repr(u8)]
pub enum AldermanMissionType {
    FoundTown = 0x00,
    OverlandTradeRoute = 0x01,
    NotoriousPirate = 0x02,
    PirateHideout = 0x03,
    SupplyProblems = 0x04,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, FromPrimitive)]
#[repr(u8)]
// Extended from https://github.com/Sqoops/PatrizierKartenEditor
pub enum TownGroundTypeId {
    // Road = 0x24, uses opcode 0x25
    Border = 0x06,
    Wall = 0x09,
    Wall2 = 0x0a,
    CoastalBuilding = 0x0b,
    Water = 0x0c,
    Unusable = 0x0d,
    FisherSite = 0x0e,
    WallAdjacent = 0x11,
    HouseSiteVertical = 0xa1,
    HouseSiteHorizontal = 0xa2,
    EnterpriseSiteInnerWallsIfAdjacent = 0x20,
    EnterpriseSite = 0x3b,
    InnerSite = 0x80,
    StreetInnerWalls = 0x83,
    StreetAdjacent = 0x84,
}

impl WareId {
    pub const fn get_scaling(&self) -> i32 {
        match self {
            WareId::Grain => 2000,
            WareId::Meat => 2000,
            WareId::Fish => 2000,
            WareId::Beer => 200,
            WareId::Salt => 200,
            WareId::Honey => 200,
            WareId::Spices => 200,
            WareId::Wine => 200,
            WareId::Cloth => 200,
            WareId::Skins => 200,
            WareId::WhaleOil => 200,
            WareId::Timber => 2000,
            WareId::IronGoods => 200,
            WareId::Leather => 200,
            WareId::Wool => 2000,
            WareId::Pitch => 200,
            WareId::PigIron => 2000,
            WareId::Hemp => 2000,
            WareId::Pottery => 200,
            WareId::Bricks => 2000,
            WareId::Sword => 10,
            WareId::Bow => 10,
            WareId::Crossbow => 10,
            WareId::Carbine => 10,
        }
    }

    pub const fn is_barrel_ware(&self) -> bool {
        match self {
            WareId::Grain => false,
            WareId::Meat => false,
            WareId::Fish => false,
            WareId::Timber => false,
            WareId::Wool => false,
            WareId::PigIron => false,
            WareId::Hemp => false,
            WareId::Bricks => false,
            WareId::Sword => panic!(),
            WareId::Bow => panic!(),
            WareId::Crossbow => panic!(),
            WareId::Carbine => panic!(),
            _ => true,
        }
    }

    pub unsafe fn get_base_price(&self) -> f32 {
        let base_price_ptr: *const f32 = 0x00673A18 as _;
        *base_price_ptr.add(*self as usize)
    }
}

impl ShipType {
    pub unsafe fn get_base_speed(&self) -> u16 {
        let base_speed_ptr: *const u16 = 0x0067366C as _;
        *base_speed_ptr.add(*self as usize)
    }
}

impl ShipWeaponId {
    pub const fn get_scaling(&self) -> u32 {
        match self {
            ShipWeaponId::SmallCatapult => 1000,
            ShipWeaponId::SmallBallista => 1000,
            ShipWeaponId::LargeCatapult => 2000,
            ShipWeaponId::LargeBallista => 2000,
            ShipWeaponId::Bombard => 2000,
            ShipWeaponId::Cannon => 1000,
        }
    }
}
