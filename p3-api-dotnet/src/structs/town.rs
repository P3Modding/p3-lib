use p3_api::{data::{ship::Ship, town::TownPtr}, p3_access_api::open_process_p3_access_api::OpenProcessP3AccessApi};

#[repr(C)]
pub struct DotnetTown {
    pub wares: [i32; 0x18],
    pub daily_consumption_businesses: [i32; 0x18],
    pub daily_production: [i32; 0x18],
}

impl DotnetTown {
    pub fn from_ptr(town: TownPtr<OpenProcessP3AccessApi>, api: &OpenProcessP3AccessApi) -> Result<Self, ()> {
        Ok(Self {
            wares: town.get_storage().get_wares(api).unwrap(),
            daily_consumption_businesses: town.get_storage().get_daily_consumptions_businesses(api).unwrap(),
            daily_production: town.get_storage().get_daily_production(api).unwrap(),
        })
    }
}
