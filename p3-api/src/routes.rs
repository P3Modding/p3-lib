use serde::{Deserialize, Serialize};

use crate::data::enums::{TownId, WareId};

#[derive(Debug, Deserialize, Serialize)]
pub struct RouteConfiguration {
    pub convoy: String,
    pub actions: Vec<RouteAction>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RouteAction {
    pub town: TownId,
    #[serde(flatten)]
    pub action: RouteTownAction,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RouteTownAction {
    Repair,
    UnloadAll,
    LoadWare { ware_id: WareId, amount: i32, locked_amount: i32 },
    FillWare { ware_id: WareId, amount: i32 },
}
