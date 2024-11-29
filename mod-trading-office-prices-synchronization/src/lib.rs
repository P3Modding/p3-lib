use log::{debug, warn};
use num_traits::cast::FromPrimitive;
use p3_api::{
    data::enums::WareId, game_world::GAME_WORLD_PTR, operation::Operation, operations::OPERATIONS_PTR, town::get_town_name,
    ui::ui_trading_office_window::UITradingOfficeWindowPtr,
};

mod ffi;

pub(crate) unsafe fn synchronize_autotrade_settings() {
    // Pull merchant index from operatons.field_924?
    let merchant_index = OPERATIONS_PTR.get_player_merchant_index();
    let window = UITradingOfficeWindowPtr::default();
    let town_index = window.get_town_index();
    let town_name = get_town_name(town_index as _).unwrap();
    debug!("synchronize_autotrade_settings from {town_name}");
    let setting_office = GAME_WORLD_PTR.get_office_in_of(town_index as _, merchant_index as _);
    let setting_office = match setting_office {
        Some(office) => office,
        None => {
            warn!("Could not find office in {town_name} of merchant {merchant_index:#x}");
            return;
        }
    };

    let setting_prices = setting_office.get_administrator_trade_prices();
    let setting_stock = setting_office.get_administrator_trade_stock();
    let merchant = GAME_WORLD_PTR.get_merchant(merchant_index as _);
    let mut office_index = merchant.get_first_office_index();
    while office_index < GAME_WORLD_PTR.get_offices_count() {
        let office = GAME_WORLD_PTR.get_office(office_index);
        if office.address == setting_office.address {
            office_index = office.get_next_office_of_merchant_index();
            continue;
        }
        let stock = office.get_administrator_trade_stock();

        for i in 0..20 {
            let ware_id = WareId::from_usize(i).unwrap();
            if setting_stock[i] / ware_id.get_scaling() % 2 == 0 {
                continue;
            }
            let stock = stock[i];
            let price = setting_prices[i];
            
            OPERATIONS_PTR.enqueue_operation(Operation::OfficeAutotradeSettingChange {
                stock,
                price,
                office_index: office_index as _,
                ware_id,
            });
        }

        office_index = office.get_next_office_of_merchant_index();
    }
}
