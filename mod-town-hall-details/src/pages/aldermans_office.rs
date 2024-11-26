use std::ffi::{CStr, CString};

use log::warn;
use p3_api::{
    data::{ddraw_set_constant_color, ui_render_text_at},
    game_world::GAME_WORLD_PTR,
    missions::alderman_missions::{AldermanMissionDataPtr, FoundTownPtr},
    operations::OPERATIONS_PTR,
    scheduled_tasks::{scheduled_task::ScheduledTaskData, SCHEDULED_TASKS_PTR},
    ui::{
        font::{self, get_normal_font},
        ui_town_hall_window::UITownHallWindowPtr,
    },
};

const COL_OFFSETS: &[i32; 2] = &[25, 400];
static TASK_RESCHEDULING_IN: &CStr = c"Rescheduling in";
static TASK_RESCHEDULES_REMAINING: &CStr = c"Reschedule Counter";
static TOWN: &CStr = c"Town";
static EFFECTIVE_PRODUCTION: &CStr = c"Effective Production";
static LOW_PRODUCTION: &CStr = c"Low Production";

pub(crate) unsafe fn draw_page(window: UITownHallWindowPtr) {
    let next_mission_index = window.get_next_mission_index();
    if next_mission_index == 0 {
        return;
    }

    if SCHEDULED_TASKS_PTR.get_merchant_alderman_mission_task_index(OPERATIONS_PTR.get_player_merchant_index()) != -1 {
        return;
    }

    let selected_mission_index = window.get_selected_alderman_mission_index();
    if selected_mission_index == 0xff {
        return;
    }
    let task_index = window.get_task_index(selected_mission_index);
    let task = SCHEDULED_TASKS_PTR.get_scheduled_task(task_index);
    let data = match task.get_data() {
        Some(e) => e,
        None => return,
    };

    let mission = match data {
        ScheduledTaskData::AldermanMission(mission) => mission,
        _ => return,
    };

    // Render general info
    ddraw_set_constant_color(0xff000000);
    font::ddraw_set_font(get_normal_font());
    let x = window.get_x();
    let mut y = window.get_y() + 60;

    font::ddraw_set_text_mode(font::TextMode::AlignLeft);
    ui_render_text_at(x + COL_OFFSETS[0], y, TASK_RESCHEDULING_IN.to_bytes());
    font::ddraw_set_text_mode(font::TextMode::AlignRight);
    let rescheduling_in = task.get_due_timestamp() - GAME_WORLD_PTR.get_game_time_raw();
    let task_due_in_cstring = CString::new(format!("{rescheduling_in:#05x}")).unwrap();
    ui_render_text_at(x + COL_OFFSETS[1], y, task_due_in_cstring.to_bytes());
    y += 20;

    font::ddraw_set_text_mode(font::TextMode::AlignLeft);
    ui_render_text_at(x + COL_OFFSETS[0], y, TASK_RESCHEDULES_REMAINING.to_bytes());
    font::ddraw_set_text_mode(font::TextMode::AlignRight);
    let reschedules_remaining = mission.get_reschedule_counter();
    let task_due_in_cstring = CString::new(format!("{reschedules_remaining}")).unwrap();
    ui_render_text_at(x + COL_OFFSETS[1], y, task_due_in_cstring.to_bytes());

    // Render mission-specific info
    match mission.get_data() {
        AldermanMissionDataPtr::FoundTownPtr(ptr) => render_aldermans_office_modifications_found_town(window, &ptr),
        AldermanMissionDataPtr::OverlandTradeRoute(_ptr) => {}
        AldermanMissionDataPtr::NotoriousPirate(_ptr) => {}
        AldermanMissionDataPtr::PirateHideout(_ptr) => {}
        AldermanMissionDataPtr::SupplyProblems(_ptr) => {}
    }
}

unsafe fn render_aldermans_office_modifications_found_town(window: UITownHallWindowPtr, data: &FoundTownPtr) {
    let town = data.get_town();
    let effective_raw = data.get_production_effective_raw();
    let effective = data.get_production_effective();
    let x = window.get_x();
    let mut y = window.get_y() + 100;

    font::ddraw_set_text_mode(font::TextMode::AlignLeft);
    ui_render_text_at(x + COL_OFFSETS[0], y, TOWN.to_bytes());
    font::ddraw_set_text_mode(font::TextMode::AlignRight);
    let town_cstring = CString::new(format!("{town:?}")).unwrap();
    ui_render_text_at(x + COL_OFFSETS[1], y, town_cstring.to_bytes());
    y += 20;

    font::ddraw_set_text_mode(font::TextMode::AlignLeft);
    ui_render_text_at(x + COL_OFFSETS[0], y, EFFECTIVE_PRODUCTION.to_bytes());
    font::ddraw_set_text_mode(font::TextMode::AlignRight);
    let mut effective_string = String::new();
    for facility in effective {
        match facility {
            p3_api::data::enums::FacilityId::Militia => warn!("Unexpected facility {facility:?}"),
            p3_api::data::enums::FacilityId::Shipyard => warn!("Unexpected facility {facility:?}"),
            p3_api::data::enums::FacilityId::Construction => warn!("Unexpected facility {facility:?}"),
            p3_api::data::enums::FacilityId::Weaponsmith => warn!("Unexpected facility {facility:?}"),
            p3_api::data::enums::FacilityId::HuntingLodge => effective_string.push_str("Skins, "),
            p3_api::data::enums::FacilityId::FishermansHouse => {
                if effective_raw & 0x20000 != 0 {
                    effective_string.push_str("Whale Oil, ")
                } else {
                    effective_string.push_str("Fish, ")
                }
            }
            p3_api::data::enums::FacilityId::Brewery => effective_string.push_str("Beer, "),
            p3_api::data::enums::FacilityId::Workshop => effective_string.push_str("Iron Goods, "),
            p3_api::data::enums::FacilityId::Apiary => effective_string.push_str("Honey, "),
            p3_api::data::enums::FacilityId::GrainFarm => effective_string.push_str("Grain, "),
            p3_api::data::enums::FacilityId::CattleFarm => effective_string.push_str("Meat, Leather, "),
            p3_api::data::enums::FacilityId::Sawmill => effective_string.push_str("Timber, "),
            p3_api::data::enums::FacilityId::WeavingMill => effective_string.push_str("Cloth, "),
            p3_api::data::enums::FacilityId::Saltery => effective_string.push_str("Salt, "),
            p3_api::data::enums::FacilityId::Ironsmelter => effective_string.push_str("Pig Iron, "),
            p3_api::data::enums::FacilityId::SheepFarm => effective_string.push_str("Wool, "),
            p3_api::data::enums::FacilityId::Vineyard => effective_string.push_str("Wine, "),
            p3_api::data::enums::FacilityId::Pottery => effective_string.push_str("Pottery, "),
            p3_api::data::enums::FacilityId::Brickworks => effective_string.push_str("Bricks, "),
            p3_api::data::enums::FacilityId::Pitchmaker => effective_string.push_str("Pitch, "),
            p3_api::data::enums::FacilityId::HempFarm => effective_string.push_str("Hemp, "),
        }
    }
    effective_string.pop();
    effective_string.pop();
    let effective_cstring = CString::new(effective_string).unwrap();
    ui_render_text_at(x + COL_OFFSETS[1], y, effective_cstring.to_bytes());
    y += 20;

    font::ddraw_set_text_mode(font::TextMode::AlignLeft);
    ui_render_text_at(x + COL_OFFSETS[0], y, LOW_PRODUCTION.to_bytes());

    font::ddraw_set_text_mode(font::TextMode::AlignRight);
    let mut ineffective_string = String::new();
    if effective_raw & 0x20000 != 0 {
        ineffective_string.push_str("Fish")
    }
}
