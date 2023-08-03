use crate::ffi;
use log::{debug, error, info, trace, warn};
use num_traits::cast::FromPrimitive;
use p3_api::{
    data::{
        class6::Class6Ptr,
        enums::{TownId, WareId},
        game_world::GameWorldPtr,
        operation::Operation,
        ship::ShipPtr,
    },
    p3_access_api::{raw_p3_access_api::RawP3AccessApi, P3AccessApi},
    strum::IntoEnumIterator,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, str::FromStr};
use std::{fs, marker::PhantomData, sync::Mutex};

static CONTEXT: Mutex<Option<RoutesContext>> = Mutex::new(None);
const P3: RawP3AccessApi = RawP3AccessApi::new();
const GAME_WORLD: GameWorldPtr<RawP3AccessApi> = GameWorldPtr::new();
const CLASS6: Class6Ptr<RawP3AccessApi> = Class6Ptr::new();

#[derive(Debug)]
pub enum LoadRouteError {
    FileError(std::io::Error),
    TomlError(toml::de::Error),
    InvalidTown(String),
    UnknownShip(String),
}

#[derive(Debug)]
pub struct RoutesContext {
    hub_routes: Vec<HubRoute>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct P3AgentConfiguration {
    hub_routes: Vec<HubRouteConfiguration>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HubRouteConfiguration {
    pub hub: String,
    pub satellite: String,
}

#[derive(Debug)]
pub struct HubRoute {
    pub hub_id: TownId,
    pub satellite_id: TownId,
    next_action: NextAction,
}

#[derive(Debug, Eq, PartialEq)]
enum NextAction {
    HubUnload,
    HubLoad,
    Satellite,
}

pub fn init_routes() -> Result<(), LoadRouteError> {
    debug!("init_routes");
    let mut mg = CONTEXT.lock().unwrap();
    let config = fs::read_to_string("p3-agent.toml").map_err(LoadRouteError::FileError)?;
    let p3_agent_config: P3AgentConfiguration = toml::from_str(&config).map_err(LoadRouteError::TomlError)?;

    *mg = Some(RoutesContext {
        hub_routes: p3_agent_config.hub_routes.iter().filter_map(|e| create_route(e).ok()).collect(),
    });
    Ok(())
}

pub fn create_route(hub_route_configuration: &HubRouteConfiguration) -> Result<HubRoute, LoadRouteError> {
    let hub_id = TownId::from_str(&hub_route_configuration.hub).or(Err(LoadRouteError::InvalidTown(hub_route_configuration.hub.clone())))?;
    let satellite_id = TownId::from_str(&hub_route_configuration.satellite).or(Err(LoadRouteError::InvalidTown(hub_route_configuration.satellite.clone())))?;
    let (ship, ship_id) = CLASS6
        .get_ship_by_name(&format!("{:?}", satellite_id), &P3)
        .unwrap()
        .ok_or_else(|| LoadRouteError::UnknownShip(format!("{:?}", satellite_id)))?;
    let destination_town = ship.get_destination_town_id(&P3).unwrap();
    // We can rely on either last_town or destination town being set.
    // To stop running vanilla traderoutes, we issue a move command to the infered destination.
    let next_action = if let Some(destination_town) = destination_town {
        if destination_town == hub_id {
            ffi::schedule_operation(&Operation::MoveShipToTown {
                ship_id: ship_id as u32,
                town: hub_id,
            });
            NextAction::HubUnload
        } else if destination_town == satellite_id {
            ffi::schedule_operation(&Operation::MoveShipToTown {
                ship_id: ship_id as u32,
                town: satellite_id,
            });
            NextAction::Satellite
        } else {
            warn!("{:?} is travelling to unexpected town {:?}", satellite_id, destination_town);
            ffi::schedule_operation(&Operation::MoveShipToTown {
                ship_id: ship_id as u32,
                town: hub_id,
            });
            NextAction::HubUnload
        }
    } else {
        warn!("{:?} is not travelling", satellite_id);
        ffi::schedule_operation(&Operation::MoveShipToTown {
            ship_id: ship_id as u32,
            town: hub_id,
        });
        NextAction::HubUnload
    };

    let route = HubRoute {
        hub_id,
        satellite_id,
        next_action,
    };
    debug!("Hub Route {:?} loaded", &route);

    Ok(route)
}

pub fn tick_routes() {
    let mut mg = CONTEXT.lock().unwrap();
    let context = mg.as_mut().unwrap();

    for route in &mut context.hub_routes {
        route.tick();
    }
}

impl HubRoute {
    fn tick(&mut self) {
        let (ship, ship_id) = match CLASS6.get_ship_by_name(&format!("{:?}", self.satellite_id), &P3).unwrap() {
            Some(s) => s,
            None => {
                error!("Could not find flagship {:?}", self.satellite_id);
                return;
            }
        };

        let convoy_id = ship.get_convoy_id(&P3).unwrap();
        let convoy = match CLASS6.get_convoy(convoy_id, &P3).unwrap() {
            Some(s) => s,
            None => {
                error!("Could not find convoy {:04x} of ship {:04x}", convoy_id, ship_id);
                return;
            }
        };

        let convoy_status = convoy.get_status(&P3).unwrap();
        if convoy_status != 0 {
            trace!("Convoy is not docked {:#04x}", convoy_status);
            return;
        }

        let ship_status = ship.get_status(&P3).unwrap();
        if ship_status != 0 {
            trace!("Ship is not docked {:#04x}", convoy_status);
            return;
        }

        match self.next_action {
            NextAction::HubUnload => self.handle_hub_unload(&ship, convoy_id),
            NextAction::HubLoad => self.handle_hub_load(ship_id, &ship, convoy_id),
            NextAction::Satellite => self.handle_satellite(ship_id, &ship, convoy_id),
        }
    }

    fn handle_hub_unload(&mut self, ship: &ShipPtr<RawP3AccessApi>, convoy_id: u16) {
        let last_town = ship.get_last_town_id(&P3).unwrap().unwrap();

        if last_town != self.hub_id {
            trace!("Ship is not on the way, but also not at {:?}", self.hub_id);
            return;
        }

        // Unload all
        debug!("{:?} unloading all", self.hub_id);
        for ware in WareId::iter() {
            ffi::schedule_operation(&Operation::MoveWaresConvoy {
                raw_amount: (i32::MAX / ware.get_scaling()) * ware.get_scaling(),
                convoy_id,
                ware,
                merchant_id: 0x24,
                to_ship: false,
            });
        }

        // Repair if needed
        if ship.get_current_health(&P3).unwrap() != ship.get_max_health(&P3).unwrap() {
            debug!("{:?} repairing", self.satellite_id);
            ffi::schedule_operation(&Operation::RepairConvoy { convoy_id: convoy_id as u32 });
        }

        self.next_action = NextAction::HubLoad;
    }

    fn handle_hub_load(&mut self, ship_id: u16, ship: &ShipPtr<RawP3AccessApi>, convoy_id: u16) {
        let current_town = ship.get_last_town_id(&P3).unwrap().unwrap();
        let hub_office = GAME_WORLD.get_office_in_of(self.hub_id, 0x24, &P3).unwrap().unwrap();
        let satellite_statistics: TownStatistics<RawP3AccessApi> = TownStatistics::new(&GAME_WORLD, self.satellite_id, &P3);

        if current_town != self.hub_id {
            trace!("Ship is not on the way, but also not at {:?}", self.hub_id);
            return;
        }

        // Load from Hub
        debug!("{:?} loading from hub", self.satellite_id);
        for ware_id in WareId::iter() {
            let weekly_consumption = satellite_statistics.get_weekly_consumption_rounded(ware_id);
            let weekly_production = satellite_statistics.get_weekly_production_rounded(ware_id);
            if weekly_consumption > weekly_production {
                debug!(
                    "{:?} weekly consumption {} NOT covered by production {}",
                    ware_id, weekly_consumption, weekly_production
                );

                let desired_amount = weekly_consumption - weekly_production;
                let locked_amount = 2 * desired_amount;
                let office_ware = hub_office.get_storage().get_ware(ware_id, &P3).unwrap() / ware_id.get_scaling();
                let available_amount = if office_ware > locked_amount { office_ware - locked_amount } else { 0 };
                let amount = if desired_amount > available_amount {
                    available_amount
                } else {
                    desired_amount
                };
                if amount == 0 {
                    warn!("Cannot satisfy {} {:?}", desired_amount, ware_id);
                    continue;
                }

                debug!("Loading {} {:?}", amount, ware_id);
                ffi::schedule_operation(&Operation::MoveWaresConvoy {
                    raw_amount: amount * ware_id.get_scaling(),
                    convoy_id,
                    ware: ware_id,
                    merchant_id: 0x24,
                    to_ship: true,
                });
            } else {
                debug!(
                    "{:?} weekly consumption {} is covered by production {}",
                    ware_id, weekly_consumption, weekly_production
                );
            }
        }

        // Travel to satellite
        debug!("Moving convoy {:#04x} to satellite {:?}", convoy_id, self.satellite_id);
        ffi::schedule_operation(&Operation::MoveShipToTown {
            ship_id: ship_id as u32,
            town: self.satellite_id,
        });
        self.next_action = NextAction::Satellite;
    }

    fn handle_satellite(&mut self, ship_id: u16, ship: &ShipPtr<RawP3AccessApi>, convoy_id: u16) {
        let satellite_office = GAME_WORLD.get_office_in_of(self.satellite_id, 0x24, &P3).unwrap().unwrap();
        let satellite_statistics: TownStatistics<RawP3AccessApi> = TownStatistics::new(&GAME_WORLD, self.satellite_id, &P3);
        let last_town_id = ship.get_last_town_id(&P3).unwrap();
        let destination_town_id = ship.get_destination_town_id(&P3).unwrap();
        if last_town_id != destination_town_id {
            return;
        }

        if last_town_id.unwrap() != self.satellite_id {
            return;
        }

        // Fill up satellite
        info!("Fill up satellite");
        for ware_id in WareId::iter() {
            let weekly_consumption = satellite_statistics.get_weekly_consumption_rounded(ware_id);
            let weekly_production = satellite_statistics.get_weekly_production_rounded(ware_id);
            if weekly_consumption <= weekly_production {
                continue;
            }

            let weekly_demand = weekly_consumption - weekly_production;
            let desired_level = 2 * weekly_demand;
            let office_ware = satellite_office.get_storage().get_ware(ware_id, &P3).unwrap() / ware_id.get_scaling();
            if desired_level > office_ware {
                let diff = desired_level - office_ware;
                debug!("{:?} unloading {} {:?}", self.satellite_id, diff, ware_id);
                ffi::schedule_operation(&Operation::MoveWaresConvoy {
                    raw_amount: diff * ware_id.get_scaling(),
                    convoy_id,
                    ware: ware_id,
                    merchant_id: 0x24,
                    to_ship: false,
                });
            }
        }

        // Load at satellite
        for ware_id in WareId::iter() {
            let weekly_consumption = satellite_statistics.get_weekly_consumption_rounded(ware_id);
            let weekly_production = satellite_statistics.get_weekly_production_rounded(ware_id);
            if weekly_production < weekly_consumption {
                continue;
            }

            debug!("Load maximum {:?} (prod={} cons={:?})", ware_id, weekly_production, weekly_consumption);
            ffi::schedule_operation(&Operation::MoveWaresConvoy {
                raw_amount: 999999 * ware_id.get_scaling(),
                convoy_id,
                ware: ware_id,
                merchant_id: 0x24,
                to_ship: true,
            });
        }

        // Travel to hub
        debug!("Moving {:?} to {:?}", self.satellite_id, self.hub_id);
        ffi::schedule_operation(&Operation::MoveShipToTown {
            ship_id: ship_id as u32,
            town: self.hub_id,
        });
        self.next_action = NextAction::HubUnload;
    }
}

struct TownStatistics<P3> {
    daily_consumptions_citizens: [i32; 24],
    daily_consumptions_town_businesses: [i32; 24],
    daily_consumptions_merchant_businesses: [i32; 24],
    daily_production_town: [i32; 24],
    daily_production_merchant: [i32; 24],
    phantom: PhantomData<P3>,
}

fn get_weekly_f32(arg: (usize, &i32)) -> f32 {
    let ware_id = WareId::from_usize(arg.0).unwrap();
    *arg.1 as f32 * 7.0 / ware_id.get_scaling() as f32
}

impl<P3: P3AccessApi> Debug for TownStatistics<P3> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TownStatistics")
            .field(
                "weekly_consumptions_citizens",
                &self.daily_consumptions_citizens.iter().enumerate().map(get_weekly_f32).collect::<Vec<f32>>(),
            )
            .field(
                "weekly_consumptions_town_businesses",
                &self
                    .daily_consumptions_town_businesses
                    .iter()
                    .enumerate()
                    .map(get_weekly_f32)
                    .collect::<Vec<f32>>(),
            )
            .field(
                "weekly_consumptions_merchant_businesses",
                &self
                    .daily_consumptions_merchant_businesses
                    .iter()
                    .enumerate()
                    .map(get_weekly_f32)
                    .collect::<Vec<f32>>(),
            )
            .field(
                "weekly_production_town",
                &self.daily_production_town.iter().enumerate().map(get_weekly_f32).collect::<Vec<f32>>(),
            )
            .field(
                "weekly_production_merchant",
                &self.daily_production_merchant.iter().enumerate().map(get_weekly_f32).collect::<Vec<f32>>(),
            )
            .finish()
    }
}

impl<P3: P3AccessApi> TownStatistics<P3> {
    pub fn new(game_world: &GameWorldPtr<P3>, town_id: TownId, api: &P3) -> Self {
        let town = game_world.get_town(town_id, api).unwrap();
        let office = game_world.get_office_in_of(town_id, 0x24, api).unwrap().unwrap();
        Self {
            daily_consumptions_citizens: town.get_daily_consumptions_citizens(api).unwrap(),
            daily_consumptions_town_businesses: town.get_storage().get_daily_consumptions_businesses(api).unwrap(),
            daily_consumptions_merchant_businesses: office.get_storage().get_daily_consumptions_businesses(api).unwrap(),
            daily_production_town: town.get_storage().get_daily_production(api).unwrap(),
            daily_production_merchant: office.get_storage().get_daily_production(api).unwrap(),
            phantom: PhantomData,
        }
    }

    pub fn get_weekly_consumption_rounded(&self, ware_id: WareId) -> i32 {
        (self.daily_consumptions_citizens[ware_id as usize]
            + self.daily_consumptions_town_businesses[ware_id as usize]
            + self.daily_consumptions_merchant_businesses[ware_id as usize])
            * 7
            / ware_id.get_scaling()
    }

    pub fn get_weekly_production_rounded(&self, ware_id: WareId) -> i32 {
        (self.daily_production_town[ware_id as usize] + self.daily_production_merchant[ware_id as usize]) * 7 / ware_id.get_scaling()
    }
}
