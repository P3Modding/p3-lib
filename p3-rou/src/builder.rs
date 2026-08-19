//! Building trade route stops, and the format semantics this encodes (all verified
//! against game-saved routes):
//!
//! - Per stop and ware, the operation is encoded by the signs of price and amount:
//!   load office -> ship = price 0, positive amount; unload ship -> office = price 0,
//!   negative amount; sell to town = positive minimum price, positive amount; buy from
//!   town = negated maximum price, positive amount.
//! - Amounts are raw units: in-game units times the ware scaling (loads 2000,
//!   barrels 200, weapons 10). [MAX_AMOUNT] means "as much as possible", unscaled.
//! - The action byte carries the repair flag ([FLAG_R]/[FLAG_X]/[FLAG_NONE]) and
//!   [FIRST_STOP_MARKER] on the route's first stop.
//! - The per-stop order array is the instruction order; [partitioned_order] puts
//!   unloading wares first so the ship frees space before taking new cargo on.

use std::str::FromStr;

use num_traits::FromPrimitive;
use p3_api::data::enums::WareId;

use crate::TradeRouteStop;

/// The game's sentinel for "as much as possible", stored unscaled.
pub const MAX_AMOUNT: i32 = 1_000_000_000;
/// Raw units per in-game unit: wares measured in loads (1 load = 10 barrels).
pub const LOAD_SCALING: i32 = 2000;
/// Raw units per in-game unit: wares measured in barrels.
pub const BARREL_SCALING: i32 = 200;
/// Set on the first stop of every route the game saves.
pub const FIRST_STOP_MARKER: u8 = 0x04;
/// Repair the ship at this stop.
pub const FLAG_R: u8 = 0x01;
pub const FLAG_X: u8 = 0x00;
pub const FLAG_NONE: u8 = 0x09;

/// The ware display order the game writes into every saved route.
pub const DEFAULT_ORDER: [u8; 24] = [
    0x03, 0x13, 0x08, 0x02, 0x00, 0x11, 0x05, 0x0c, 0x0d, 0x01, 0x10, 0x0f, 0x12, 0x04, 0x09, 0x06, 0x0b, 0x0a, 0x07, 0x0e, 0x15, 0x17, 0x16, 0x14,
];

/// Looks up a ware by its exact `WareId` identifier (e.g. "PigIron").
pub fn ware_index(name: &str) -> Option<usize> {
    WareId::from_str(name).ok().map(|ware| ware as usize)
}

/// The ware's raw-unit scaling (raw units per in-game unit), from p3-api.
pub fn ware_scaling(index: usize) -> i32 {
    WareId::from_usize(index).map(|ware| ware.get_scaling()).unwrap_or(1)
}

/// A stop in the default instruction order.
pub fn stop(town_index: u8, action: u8, price: [i32; 24], amount: [i32; 24]) -> TradeRouteStop {
    TradeRouteStop {
        town_index,
        action,
        order: DEFAULT_ORDER,
        price,
        amount,
    }
}

/// The order array is the stop's instruction order; put the unloading wares first so
/// the ship frees up space before taking new cargo on.
pub fn partitioned_order(unloads_first: [i32; 24]) -> [u8; 24] {
    let mut order = [0u8; 24];
    let mut n = 0;
    for &ware in DEFAULT_ORDER.iter().filter(|&&w| unloads_first[w as usize] < 0) {
        order[n] = ware;
        n += 1;
    }
    for &ware in DEFAULT_ORDER.iter().filter(|&&w| unloads_first[w as usize] >= 0) {
        order[n] = ware;
        n += 1;
    }
    order
}

/// Sell without an amount limit at the given minimum prices, and take back exactly the
/// given amounts, for every ware with a nonzero load amount.
fn sell_max_and_unload(load_amount: &[i32; 24]) -> ([i32; 24], [i32; 24]) {
    let mut sell_max = [0i32; 24];
    let mut unload_calculated = [0i32; 24];
    for (i, load) in load_amount.iter().enumerate() {
        if *load != 0 {
            sell_max[i] = MAX_AMOUNT;
            unload_calculated[i] = -load;
        }
    }
    (sell_max, unload_calculated)
}

/// Supply route: load the given raw amounts at the source (repairing there), sell max
/// at the given minimum prices in the target town, reset the target office stock to
/// exactly the loaded amounts (take everything, put the amounts back), and unload the
/// whole ship into the source office.
pub fn five_stop_route(load_town: u8, sell_town: u8, load_amount: [i32; 24], sell_prices: [i32; 24]) -> Vec<TradeRouteStop> {
    let (sell_max, unload_calculated) = sell_max_and_unload(&load_amount);
    vec![
        stop(load_town, FLAG_R | FIRST_STOP_MARKER, [0i32; 24], load_amount),
        stop(sell_town, FLAG_X, sell_prices, sell_max),
        stop(sell_town, FLAG_X, [0i32; 24], [MAX_AMOUNT; 24]),
        stop(sell_town, FLAG_X, [0i32; 24], unload_calculated),
        stop(load_town, FLAG_X, [0i32; 24], [-MAX_AMOUNT; 24]),
    ]
}

/// Like [five_stop_route], but swaps the target office stock one unit category at a
/// time (unload loads-goods + take barrels, then unload barrel amounts + take
/// loads-goods, then unload loads amounts), which bounds the ship space the shuffle
/// needs. Weapons take part only in the initial load, the sale, and the final unload.
pub fn six_stop_route(load_town: u8, sell_town: u8, load_amount: [i32; 24], sell_prices: [i32; 24]) -> Vec<TradeRouteStop> {
    let (sell_max, unload_calculated) = sell_max_and_unload(&load_amount);
    let mut unload_loads_take_barrels = [0i32; 24];
    let mut unload_barrels_take_loads = [0i32; 24];
    let mut unload_loads_calculated = [0i32; 24];
    for i in 0..24 {
        match ware_scaling(i) {
            LOAD_SCALING => {
                unload_loads_take_barrels[i] = -MAX_AMOUNT;
                unload_barrels_take_loads[i] = MAX_AMOUNT;
                unload_loads_calculated[i] = unload_calculated[i];
            }
            BARREL_SCALING => {
                unload_loads_take_barrels[i] = MAX_AMOUNT;
                unload_barrels_take_loads[i] = unload_calculated[i];
            }
            _ => {}
        }
    }
    let mut swap_barrels_stop = stop(sell_town, FLAG_X, [0i32; 24], unload_loads_take_barrels);
    swap_barrels_stop.order = partitioned_order(unload_loads_take_barrels);
    let mut swap_loads_stop = stop(sell_town, FLAG_X, [0i32; 24], unload_barrels_take_loads);
    swap_loads_stop.order = partitioned_order(unload_barrels_take_loads);
    vec![
        stop(load_town, FLAG_R | FIRST_STOP_MARKER, [0i32; 24], load_amount),
        stop(sell_town, FLAG_X, sell_prices, sell_max),
        swap_barrels_stop,
        swap_loads_stop,
        stop(sell_town, FLAG_X, [0i32; 24], unload_loads_calculated),
        stop(load_town, FLAG_X, [0i32; 24], [-MAX_AMOUNT; 24]),
    ]
}

/// Collection route parked in one town: unload everything into the office (with the
/// repair flag), then five stops buying max of every ware with a positive maximum
/// price in `buy_prices`.
pub fn suck_route(town: u8, buy_prices: [i32; 24]) -> Vec<TradeRouteStop> {
    let mut prices = [0i32; 24];
    let mut buy_max = [0i32; 24];
    for (i, &price) in buy_prices.iter().enumerate() {
        if price > 0 {
            prices[i] = -price;
            buy_max[i] = MAX_AMOUNT;
        }
    }
    let mut stops = vec![stop(town, FLAG_R | FIRST_STOP_MARKER, [0i32; 24], [-MAX_AMOUNT; 24])];
    for _ in 0..5 {
        stops.push(stop(town, FLAG_X, prices, buy_max));
    }
    stops
}
