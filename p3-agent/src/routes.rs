use crate::ffi;
use glob::glob;
use log::{debug, error, trace};
use p3_api::{
    data::{class6::Class6Ptr, enums::WareId, game_world::GameWorldPtr, operation::Operation},
    p3_access_api::raw_p3_access_api::RawP3AccessApi,
    routes::{RouteConfiguration, RouteTownAction},
    strum::IntoEnumIterator,
};
use std::{fs, path::PathBuf, sync::Mutex};

static CONTEXT: Mutex<Option<RoutesContext>> = Mutex::new(None);
const P3: RawP3AccessApi = RawP3AccessApi::new();

#[derive(Debug)]
pub enum LoadRouteError {
    FileError(std::io::Error),
    TomlError(toml::de::Error),
}

pub fn init_routes() -> Result<(), LoadRouteError> {
    debug!("init_routes");
    let mut mg = CONTEXT.lock().unwrap();
    let mut routes = vec![];
    for entry in glob("*.toml").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => match init_route(&path) {
                Ok(configuration) => routes.push(Route {
                    configuration,
                    next_action_id: 0,
                    wait_for_repair: false,
                }),
                Err(e) => error!("Failed to init route {:?}: {:?}", path, e),
            },
            Err(e) => error!("{:?}", e),
        }
    }
    *mg = Some(RoutesContext { routes });
    Ok(())
}

fn init_route(input_file: &PathBuf) -> Result<RouteConfiguration, LoadRouteError> {
    debug!("init route {:?}", input_file);
    let config = fs::read_to_string(input_file).map_err(LoadRouteError::FileError)?;
    toml::from_str(&config).map_err(LoadRouteError::TomlError)
}

pub fn tick_routes() {
    let mut mg = CONTEXT.lock().unwrap();
    let context = mg.as_mut().unwrap();

    for route in &mut context.routes {
        route.tick();
    }
}

struct RoutesContext {
    routes: Vec<Route>,
}

struct Route {
    configuration: RouteConfiguration,
    next_action_id: usize,
    wait_for_repair: bool,
}

pub trait RouteImpl {
    fn tick(&mut self);
}

impl RouteImpl for Route {
    fn tick(&mut self) {
        let game_world: GameWorldPtr<RawP3AccessApi> = GameWorldPtr::default();
        let class6: Class6Ptr<RawP3AccessApi> = Class6Ptr::default();
        let action = &self.configuration.actions[self.next_action_id];
        let (ship, ship_id) = match class6.get_ship_by_name(&self.configuration.convoy, &P3).unwrap() {
            Some(s) => s,
            None => {
                error!("Could not find flagship");
                return;
            }
        };

        let convoy_id = ship.get_convoy_id(&P3).unwrap();
        let convoy = match class6.get_convoy(convoy_id, &P3).unwrap() {
            Some(s) => s,
            None => {
                error!("Could not find convoy {:04x} of ship {:04x}", ship_id, convoy_id);
                return;
            }
        };

        let convoy_status = convoy.get_status(&P3).unwrap();
        if convoy_status != 0 {
            trace!("Convoy is not docked {:#04x}", convoy_status);
            return;
        }

        if self.wait_for_repair {
            if ship.get_current_health(&P3).unwrap() != ship.get_max_health(&P3).unwrap() {
                return;
            }

            self.wait_for_repair = false;
        }

        let current_town = ship.get_current_town_id(&P3).unwrap().unwrap();
        if current_town != action.town {
            debug!("Moving convoy {:#04x} to {:?}", convoy_id, action.town);
            ffi::schedule_operation(&Operation::MoveShipToTown {
                ship_id: ship_id as u32,
                town: action.town,
            });
            return;
        }

        debug!("Executing {:?}", &action);
        match &action.action {
            RouteTownAction::Repair => {
                self.wait_for_repair = true;
                ffi::schedule_operation(&Operation::RepairConvoy { convoy_id: convoy_id as u32 })
            }
            RouteTownAction::UnloadAll => {
                for ware in WareId::iter() {
                    ffi::schedule_operation(&Operation::MoveWaresConvoy {
                        amount: (i32::MAX / ware.get_scaling()) * ware.get_scaling(),
                        convoy_id,
                        ware,
                        merchant_id: 0x24,
                        to_ship: false,
                    });
                }
            }
            RouteTownAction::LoadWare {
                ware_id,
                amount,
                locked_amount,
            } => {
                let office = game_world.get_office_in_of(action.town, 0x24, &P3).unwrap().unwrap();
                let office_ware = office.get_storage().get_ware(*ware_id, &P3).unwrap() / ware_id.get_scaling();
                let available_amount = office_ware - locked_amount;
                let amount = if *amount > available_amount { available_amount } else { *amount };

                debug!("{} loading {} {:?} from office {:?}", self.configuration.convoy, amount, ware_id, action.town);
                ffi::schedule_operation(&Operation::MoveWaresConvoy {
                    amount: amount * ware_id.get_scaling(),
                    convoy_id,
                    ware: *ware_id,
                    merchant_id: 0x24,
                    to_ship: true,
                });
            }
            RouteTownAction::FillWare { ware_id, amount } => {
                let office = game_world.get_office_in_of(action.town, 0x24, &P3).unwrap().unwrap();
                let office_ware = office.get_storage().get_ware(*ware_id, &P3).unwrap() / ware_id.get_scaling();
                if *amount > office_ware {
                    let diff = amount - office_ware;
                    debug!("{} unloading {} {:?} to office {:?}", self.configuration.convoy, diff, ware_id, action.town);
                    ffi::schedule_operation(&Operation::MoveWaresConvoy {
                        amount: diff * ware_id.get_scaling(),
                        convoy_id,
                        ware: *ware_id,
                        merchant_id: 0x24,
                        to_ship: false,
                    });
                }
            }
        }
        self.next_action_id = (self.next_action_id + 1) % self.configuration.actions.len();
    }
}
