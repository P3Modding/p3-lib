use std::{
    ffi::{CStr, CString},
    mem,
    sync::atomic::Ordering,
};

use log::debug;
use num_traits::cast::FromPrimitive;
use p3_api::{
    data::{class48::Class48Ptr, ddraw_set_constant_color, ddraw_set_text_mode, enums::WareId, fill_p3_string, render_window_title, ui_render_text_at},
    game_world::GAME_WORLD_PTR,
    ui::{font, ui_town_hall_sidemenu::UITownHallSidemenuPtr, ui_town_hall_window::UITownHallWindowPtr},
};

pub(crate) mod ffi;

pub static TITLE: &CStr = c"Details";
pub static STOCK: &CStr = c"Stock";
pub static CONSUMPTION: &CStr = c"Consumption";
pub static RATIO: &CStr = c"Ratio";

#[derive(Debug, Default)]
struct HanseaticWareData {
    total_wares: i32,
    total_consumption: i32,
    ware: usize,
}

impl HanseaticWareData {
    fn get_ratio(&self) -> i32 {
        if self.total_consumption >= 1024 {
            self.total_wares / (self.total_consumption >> 10)
        } else {
            0
        }
    }
}

pub(crate) unsafe fn handle_open(ui_town_hall_window_address: u32) {
    debug!("town_hall_open_hook");
    let orig_address = (*ffi::TOWN_HALL_WINDOW_OPEN_HOOK_PTR.load(Ordering::SeqCst)).old_absolute;
    let orig: extern "thiscall" fn(aa1: u32) = mem::transmute(orig_address);
    orig(ui_town_hall_window_address);

    let class48 = Class48Ptr::new();
    class48.set_ignore_below_gradient(0);
    class48.set_gradient_y(0);
}

pub(crate) unsafe fn handle_selected_page_switch() -> i32 {
    let window = UITownHallWindowPtr::new();
    let sidemenu = UITownHallSidemenuPtr::default();
    let class48 = Class48Ptr::new();
    let selected_page = window.get_selected_page();

    if selected_page == -1 {
        let mut hanse_data: [HanseaticWareData; 20] = Default::default();
        let towns_count = GAME_WORLD_PTR.get_towns_count();

        for town_index in 0..towns_count {
            let town = GAME_WORLD_PTR.get_town(town_index as _);
            let stock = town.get_storage().get_wares();
            let consumption_citizens = town.get_daily_consumptions_citizens();
            let consumption_businesses = town.get_storage().get_daily_consumptions_businesses();
            for ware in 0..20 {
                hanse_data[ware].total_wares += stock[ware];
                hanse_data[ware].total_consumption += consumption_citizens[ware];
                hanse_data[ware].total_consumption += consumption_businesses[ware];
                hanse_data[ware].ware = ware;
            }

            let mut office_index = town.get_first_office_index();
            while office_index < GAME_WORLD_PTR.get_offices_count() {
                let office = GAME_WORLD_PTR.get_office(office_index);
                let office_stock = office.get_storage().get_wares();
                let office_consumption = office.get_storage().get_daily_consumptions_businesses();
                for ware in 0..20 {
                    hanse_data[ware].total_wares += office_stock[ware];
                    hanse_data[ware].total_consumption += office_consumption[ware];
                }
                office_index = office.next_office_in_town_index();
            }
        }
        //hanse_data.sort_by(|a, b| a.get_ratio().cmp(&b.get_ratio()));
        hanse_data.sort_by_key(|a| a.get_ratio());

        let mut title_p3_string: u32 = 0;
        fill_p3_string((&mut title_p3_string) as *mut _ as _, TITLE.to_bytes());
        render_window_title(title_p3_string as _, window.address as _);

        ddraw_set_constant_color(0xff000000);
        ddraw_set_text_mode(2);
        font::ddraw_set_font(font::get_normal_font());
        let x = window.get_x() + 80;
        let mut y = window.get_y() + 60;

        font::ddraw_set_font(font::get_header_font());
        ui_render_text_at(x + 80, y, STOCK.to_bytes());
        ui_render_text_at(x + 160, y, CONSUMPTION.to_bytes());
        ui_render_text_at(x + 220, y, RATIO.to_bytes());
        y += 20;

        for data in &hanse_data {
            let ware = WareId::from_usize(data.ware).unwrap();
            ui_render_text_at(x, y, CString::new(format!("{ware:?}")).unwrap().to_bytes());
            ui_render_text_at(x + 80, y, CString::new(format!("{}", data.total_wares)).unwrap().to_bytes());
            ui_render_text_at(x + 160, y, CString::new(format!("{}", data.total_consumption)).unwrap().to_bytes());
            if data.total_consumption >= 1024 {
                let ratio = data.get_ratio();
                ui_render_text_at(x + 220, y, CString::new(format!("{}", ratio)).unwrap().to_bytes());
            }
            y += 20;
        }
    } else if selected_page == 7 {
        // The gradient changes should be done before content rendering is done, but this works for now
        sidemenu.set_window_needs_redraw();
        class48.set_ignore_below_gradient(0);
        class48.set_gradient_y(0);
        ffi::render_aldermans_office_modifications(&window);
    }

    selected_page
}
