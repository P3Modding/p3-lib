use std::ffi::{CStr, CString};

use p3_api::{
    data::{ddraw_set_constant_color, ui_render_text_at},
    game_world::GAME_WORLD_PTR,
    missions::alderman_missions::{AldermanMissionDataPtr, FoundTownPtr},
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

pub(crate) unsafe fn draw_page(window: UITownHallWindowPtr) {
    let next_mission_index = window.get_next_mission_index();
    if next_mission_index == 0 {
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
        effective_string.push_str(&format!("{facility:?}, "));
    }
    effective_string.pop();
    effective_string.pop();
    let effective_cstring = CString::new(effective_string).unwrap();
    ui_render_text_at(x + COL_OFFSETS[1], y, effective_cstring.to_bytes());
}
