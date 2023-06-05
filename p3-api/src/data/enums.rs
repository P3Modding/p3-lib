use num_derive::FromPrimitive;
use strum::EnumIter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, FromPrimitive)]
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
    WhaleOil = 0x0A,
    Timber = 0x0B,
    IronGoods = 0x0C,
    Leather = 0x0D,
    Wool = 0x0E,
    Pitch = 0x0F,
    PigIron = 0x10,
    Hemp = 0x11,
    Pottery = 0x12,
    Bricks = 0x13,
    Sword = 0x14,
    Bow = 0x15,
    Crossbow = 0x16,
    Carbine = 0x17,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter, FromPrimitive)]
pub enum TownId {
    Edinburgh = 0x00,
    Scarborough = 0x01,
    London = 0x02,
    Bruges = 0x03,
    Groningen = 0x04,
    Cologne = 0x05,
    Bremen = 0x06,
    Ripen = 0x07,
    Hamburg = 0x08,
    Luebeck = 0x09,
    Rostock = 0x0A,
    Bergen = 0x0B,
    Oslo = 0x0C,
    Aalborg = 0x0D,
    Malmoe = 0x0E,
    Stockholm = 0x0F,
    Visby = 0x10,
    Stettin = 0x11,
    Gdansk = 0x12,
    Torun = 0x13,
    Riga = 0x14,
    Reval = 0x15,
    Ladoga = 0x16,
    Novgorod = 0x17,
    Koenigsberg = 0x18,
    Newcastle = 0x19,
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

impl WareId {
    pub fn get_scaling(&self) -> u32 {
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
}

impl ShipWeaponId {
    pub fn get_scaling(&self) -> u32 {
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
