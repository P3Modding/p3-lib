use crate::data::{
    enums::{AldermanMissionType, FacilityId, TownId},
    p3_ptr::P3Pointer,
};
use num_traits::cast::FromPrimitive;

pub struct AldermanMissionPtr {
    address: u32,
}

pub struct FoundTownPtr {
    address: u32,
}

pub struct OverlandTradeRoutePtr {
    address: u32,
}

pub struct NotoriousPiratePtr {
    address: u32,
}

pub struct PirateHideoutPtr {
    address: u32,
}

pub struct SupplyProblemsPtr {
    address: u32,
}

pub enum AldermanMissionDataPtr {
    FoundTownPtr(FoundTownPtr),
    OverlandTradeRoute(OverlandTradeRoutePtr),
    NotoriousPirate(NotoriousPiratePtr),
    PirateHideout(PirateHideoutPtr),
    SupplyProblems(SupplyProblemsPtr),
}

impl AldermanMissionPtr {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }

    pub unsafe fn get_mission_type(&self) -> AldermanMissionType {
        self.get(0x04)
    }

    pub unsafe fn get_merchant_index(&self) -> u8 {
        self.get(0x05)
    }

    pub unsafe fn get_reschedule_counter(&self) -> u8 {
        self.get(0x06)
    }

    pub unsafe fn get_data(&self) -> AldermanMissionDataPtr {
        match self.get_mission_type() {
            AldermanMissionType::FoundTown => AldermanMissionDataPtr::FoundTownPtr(FoundTownPtr::new(self.address + 0x08)),
            AldermanMissionType::OverlandTradeRoute => AldermanMissionDataPtr::OverlandTradeRoute(OverlandTradeRoutePtr::new(self.address + 0x08)),
            AldermanMissionType::NotoriousPirate => AldermanMissionDataPtr::NotoriousPirate(NotoriousPiratePtr::new(self.address + 0x08)),
            AldermanMissionType::PirateHideout => AldermanMissionDataPtr::PirateHideout(PirateHideoutPtr::new(self.address + 0x08)),
            AldermanMissionType::SupplyProblems => AldermanMissionDataPtr::SupplyProblems(SupplyProblemsPtr::new(self.address + 0x08)),
        }
    }
}

impl FoundTownPtr {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }

    pub unsafe fn get_production_effective_raw(&self) -> u32 {
        self.get(0x00)
    }

    pub unsafe fn get_production_effective(&self) -> Vec<FacilityId> {
        let mut wares = vec![];
        let raw = self.get_production_effective_raw();
        for i in 0..17 {
            if raw & 1 << i != 0 {
                wares.push(FacilityId::from_usize(i + 4).unwrap());
            }
        }
        wares
    }

    pub unsafe fn get_town(&self) -> TownId {
        self.get(0x04)
    }
}

impl OverlandTradeRoutePtr {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }
}

impl NotoriousPiratePtr {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }
}

impl PirateHideoutPtr {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }
}

impl SupplyProblemsPtr {
    pub const fn new(address: u32) -> Self {
        Self { address }
    }
}

impl P3Pointer for AldermanMissionPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

impl P3Pointer for FoundTownPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

impl P3Pointer for OverlandTradeRoutePtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

impl P3Pointer for NotoriousPiratePtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

impl P3Pointer for PirateHideoutPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}

impl P3Pointer for SupplyProblemsPtr {
    fn get_address(&self) -> u32 {
        self.address
    }
}
