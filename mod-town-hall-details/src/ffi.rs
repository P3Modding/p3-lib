use std::{
    arch::global_asm,
    ffi::{c_void, CStr, CString},
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

use hooklet::{FunctionPointerHook, X86Rel32Type};
use log::debug;
use p3_api::{
    data::{class48::Class48Ptr, ddraw_set_constant_color, ddraw_set_text_mode, ui_render_text_at},
    game_world::GAME_WORLD_PTR,
    missions::alderman_missions::{AldermanMissionDataPtr, FoundTownPtr},
    scheduled_tasks::{
        scheduled_task::{ScheduledTaskData, ScheduledTaskPtr},
        SCHEDULED_TASKS_PTR,
    },
    ui::{
        font::{self, get_normal_font},
        ui_town_hall_window::UITownHallWindowPtr,
    },
};
use windows::core::PCSTR;

pub static TOWN: &CStr = c"Town";
pub static TASK_DUE_IN: &CStr = c"Rescheduling in";
pub static EFFECTIVE_PRODUCTION: &CStr = c"Effective Production";

const TOWN_HALL_WINDOW_OPEN_POINTER_OFFSET: u32 = UITownHallWindowPtr::VTABLE_OFFSET + 0x120;
pub static TOWN_HALL_WINDOW_OPEN_HOOK_PTR: AtomicPtr<FunctionPointerHook> = AtomicPtr::new(std::ptr::null_mut());

const LOAD_TOWN_HALL_SELECTED_PAGE_PATCH_ADDRESS: u32 = 0x005E09AC;
static LOAD_TOWN_HALL_SELECTED_PAGE_CONTINUATION: u32 = 0x005E09B2;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    debug!("Hooking town hall's open function through vtable");
    match hooklet::hook_function_pointer(
        PCSTR::from_raw(ptr::null()),
        TOWN_HALL_WINDOW_OPEN_POINTER_OFFSET,
        town_hall_open_hook as usize as u32,
    ) {
        Ok(hook) => {
            debug!("Hook {hook:?} set");
            TOWN_HALL_WINDOW_OPEN_HOOK_PTR.store(Box::into_raw(Box::new(hook)), Ordering::SeqCst);
        }
        Err(_) => {
            return 1;
        }
    }

    debug!("Detouring town hall's rendering function at the selected page switch");
    if hooklet::deploy_rel32_raw(
        LOAD_TOWN_HALL_SELECTED_PAGE_PATCH_ADDRESS as _,
        (&load_town_hall_selected_page_detour) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 2;
    }

    0
}

#[no_mangle]
pub unsafe extern "thiscall" fn town_hall_open_hook(ui_town_hall_window_address: u32) {
    crate::handle_open(ui_town_hall_window_address)
}

#[no_mangle]
pub unsafe extern "thiscall" fn town_hall_rendering_hook() -> i32 {
    crate::handle_selected_page_switch()
}

pub unsafe fn render_aldermans_office_modifications(window: &UITownHallWindowPtr) {
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
    match mission.get_data() {
        AldermanMissionDataPtr::FoundTownPtr(ptr) => render_aldermans_office_modifications_found_town(window, &task, &ptr),
        AldermanMissionDataPtr::OverlandTradeRoute(_ptr) => {}
        AldermanMissionDataPtr::NotoriousPirate(_ptr) => {}
        AldermanMissionDataPtr::PirateHideout(_ptr) => {}
        AldermanMissionDataPtr::SupplyProblems(_ptr) => {}
    }
}

unsafe fn render_aldermans_office_modifications_found_town(window: &UITownHallWindowPtr, task: &ScheduledTaskPtr, data: &FoundTownPtr) {
    let town = data.get_town();
    let effective = data.get_production_effective();
    let class48 = Class48Ptr::new();
    class48.set_ignore_below_gradient(0);
    class48.set_gradient_y(0);

    ddraw_set_constant_color(0xff000000);
    ddraw_set_text_mode(2);
    font::ddraw_set_font(get_normal_font());
    let x = window.get_x();
    let mut y = window.get_y() + 60;

    ui_render_text_at(x + 120, y, TASK_DUE_IN.to_bytes());
    let due_in = task.get_due_timestamp() - GAME_WORLD_PTR.get_game_time_raw();
    let task_due_in_cstring = CString::new(format!("{due_in}")).unwrap();
    ui_render_text_at(x + 420, y, task_due_in_cstring.to_bytes());
    y += 20;

    ui_render_text_at(x + 120, y, TOWN.to_bytes());
    let town_cstring = CString::new(format!("{town:?}")).unwrap();
    ui_render_text_at(x + 420, y, town_cstring.to_bytes());
    y += 20;

    ui_render_text_at(x + 120, y, EFFECTIVE_PRODUCTION.to_bytes());
    let mut effective_string = String::new();
    for facility in effective {
        effective_string.push_str(&format!("{facility:?}, "));
    }
    effective_string.pop();
    effective_string.pop();
    let effective_cstring = CString::new(effective_string).unwrap();
    ui_render_text_at(x + 420, y, effective_cstring.to_bytes());
}

extern "C" {
    static load_town_hall_selected_page_detour: c_void;
}

global_asm!("
.global {load_town_hall_selected_page_detour}
{load_town_hall_selected_page_detour}:
# save regs
push ecx
push edx

call {town_hall_rendering_hook}

# restore regs
pop edx
pop ecx

jmp [{continuation}]
",
load_town_hall_selected_page_detour = sym load_town_hall_selected_page_detour,
town_hall_rendering_hook = sym town_hall_rendering_hook,
continuation = sym LOAD_TOWN_HALL_SELECTED_PAGE_CONTINUATION);
