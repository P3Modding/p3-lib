use std::{
    arch::global_asm,
    ffi::{c_void, CStr, CString},
};

use hooklet::X86Rel32Type;
use log::debug;
use p3_api::{
    data::{
        class48::Class48Ptr, ddraw_set_constant_color, ddraw_set_text_mode, fill_p3_string, render_window_title, statics::get_shipyard_level_requirements,
        ui_render_text_at,
    },
    game_world::GAME_WORLD_PTR,
    ui::{
        font::{self, get_header_font, get_normal_font},
        ui_shipyard_window::UIShipyardWindowPtr,
    },
};

pub static TITLE: &CStr = c"Details";
pub static EMPLOYEES: &CStr = c"Employees";
pub static EXPERIENCE: &CStr = c"Experience (scaled)";
pub static PENDING_EXPERIENCE: &CStr = c"Pending Experience (scaled)";
pub static UTILIZATION_MARKUP: &CStr = c"Utilization Markup";
pub static SNAIKKA: &CStr = c"Snaikka";
pub static CRAYER: &CStr = c"Crayer";
pub static COG: &CStr = c"Cog";
pub static HOLK: &CStr = c"Holk";
pub static QUALITY_LEVEL: &CStr = c"Quality Level";
pub static REQUIRED_XP: &CStr = c"Required XP";

const SHIPYARD_WINDOW_OPEN_PATCH_ADDRESS: u32 = 0x005ADA56;
static SHIPYARD_WINDOW_OPEN_CONTINUATION: u32 = 0x005ADA93;

const LOAD_SHIPYARD_SELECTED_PAGE_PATCH_ADDRESS: u32 = 0x005F4320;
static LOAD_SHIPYARD_SELECTED_PAGE_CONTINUATION: u32 = 0x005F4326;

const COL_OFFSETS_4: &[i32; 4] = &[60, 120, 180, 240];

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    if hooklet::deploy_rel32_raw(
        SHIPYARD_WINDOW_OPEN_PATCH_ADDRESS as _,
        (&ui_shipyard_window_open_detour) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 1;
    }

    if hooklet::deploy_rel32_raw(
        LOAD_SHIPYARD_SELECTED_PAGE_PATCH_ADDRESS as _,
        (&load_shipyard_selected_page_detour) as *const _ as _,
        X86Rel32Type::Jump,
    )
    .is_err()
    {
        return 2;
    }

    0
}

#[no_mangle]
pub unsafe extern "thiscall" fn shipyard_window_open_hook() {
    debug!("shipyard_window_open_hook");
    let class48 = Class48Ptr::new();
    class48.set_ignore_below_gradient(0);
    class48.set_gradient_y(200);
    class48.clip_stuff();
}

#[no_mangle]
pub unsafe extern "thiscall" fn shipyard_rendering_hook() -> i32 {
    debug!("shipyard_rendering_hook");
    let window = UIShipyardWindowPtr::new();
    let town_index = window.get_town_index();
    let selected_page = window.get_selected_page();
    if selected_page == -1 {
        let reqs = get_shipyard_level_requirements();
        let town = GAME_WORLD_PTR.get_town(town_index as _);
        let shipyard_facility = town.get_facility(1);
        let shipyard = town.get_shipyard();
        let current_quality_levels = shipyard.get_current_quality_levels();
        let class48 = Class48Ptr::new();
        let next_snaikka_level: usize = (current_quality_levels.snaikka_level + 1) as _;
        let next_crayer_level: usize = (current_quality_levels.craier_level + 1) as _;
        let next_cog_level: usize = (current_quality_levels.cog_level + 1) as _;
        let next_holk_level: usize = (current_quality_levels.holk_level + 1) as _;

        class48.set_ignore_below_gradient(0);
        class48.set_gradient_y(200);

        let mut title_p3_string: u32 = 0;
        fill_p3_string((&mut title_p3_string) as *mut _ as _, TITLE.to_bytes());
        render_window_title(title_p3_string as _, window.address as _);

        ddraw_set_constant_color(0xff000000);
        ddraw_set_text_mode(2);
        font::ddraw_set_font(get_normal_font());
        let x = window.get_x() + 200;
        let mut y = window.get_y() + 200;

        ui_render_text_at(x, y, EMPLOYEES.to_bytes());
        let employees_cstring = CString::new(format!("{}", shipyard_facility.get_employees())).unwrap();
        ui_render_text_at(x + 100, y, employees_cstring.to_bytes());
        y += 20;

        ui_render_text_at(x, y, UTILIZATION_MARKUP.to_bytes());
        let utilization_markup_cstring = CString::new(format!("{:.2}", shipyard.get_utilization_markup())).unwrap();
        ui_render_text_at(x + 100, y, utilization_markup_cstring.to_bytes());
        y += 20;

        ui_render_text_at(x, y, PENDING_EXPERIENCE.to_bytes());
        let pending_experience_cstring = CString::new(format!("{:.2}", shipyard.get_pending_experience() as f32 / 2800.0)).unwrap();
        ui_render_text_at(x + 100, y, pending_experience_cstring.to_bytes());
        y += 20;

        ui_render_text_at(x, y, EXPERIENCE.to_bytes());
        let experience_cstring = CString::new(format!("{:.2}", shipyard.get_experience() as f32 / 2800.0)).unwrap();
        ui_render_text_at(x + 100, y, experience_cstring.to_bytes());
        y += 20;

        font::ddraw_set_font(get_header_font());
        let table_x = window.get_x() + 100;
        ui_render_text_at(table_x + COL_OFFSETS_4[0], y, SNAIKKA.to_bytes());
        ui_render_text_at(table_x + COL_OFFSETS_4[1], y, CRAYER.to_bytes());
        ui_render_text_at(table_x + COL_OFFSETS_4[2], y, COG.to_bytes());
        ui_render_text_at(table_x + COL_OFFSETS_4[3], y, HOLK.to_bytes());
        y += 20;

        font::ddraw_set_font(get_normal_font());
        ui_render_text_at(table_x, y, QUALITY_LEVEL.to_bytes());
        let snaikka_cstring = CString::new(format!("{}", current_quality_levels.snaikka_level)).unwrap();
        ui_render_text_at(table_x + COL_OFFSETS_4[0], y, snaikka_cstring.to_bytes());
        let crayer_cstring = CString::new(format!("{}", current_quality_levels.craier_level)).unwrap();
        ui_render_text_at(table_x + COL_OFFSETS_4[1], y, crayer_cstring.to_bytes());
        let cog_cstring = CString::new(format!("{}", current_quality_levels.cog_level)).unwrap();
        ui_render_text_at(table_x + COL_OFFSETS_4[2], y, cog_cstring.to_bytes());
        let holk_cstring = CString::new(format!("{}", current_quality_levels.holk_level)).unwrap();
        ui_render_text_at(table_x + COL_OFFSETS_4[3], y, holk_cstring.to_bytes());
        y += 20;

        ui_render_text_at(table_x, y, REQUIRED_XP.to_bytes());
        if current_quality_levels.snaikka_level < 3 {
            let snaikka_cstring = CString::new(format!("{}", reqs.snaikka[next_snaikka_level])).unwrap();
            ui_render_text_at(table_x + COL_OFFSETS_4[0], y, snaikka_cstring.to_bytes());
        }
        if current_quality_levels.craier_level < 3 {
            let crayer_cstring = CString::new(format!("{}", reqs.crayer[next_crayer_level])).unwrap();
            ui_render_text_at(table_x + COL_OFFSETS_4[1], y, crayer_cstring.to_bytes());
        }
        if current_quality_levels.cog_level < 3 {
            let cog_cstring = CString::new(format!("{}", reqs.cog[next_cog_level])).unwrap();
            ui_render_text_at(table_x + COL_OFFSETS_4[2], y, cog_cstring.to_bytes());
        }
        if current_quality_levels.holk_level < 3 {
            let holk_cstring = CString::new(format!("{}", reqs.holk[next_holk_level])).unwrap();
            ui_render_text_at(table_x + COL_OFFSETS_4[3], y, holk_cstring.to_bytes());
        }
    }

    selected_page
}

extern "C" {
    static ui_shipyard_window_open_detour: c_void;
    static load_shipyard_selected_page_detour: c_void;
}

global_asm!("
.global {ui_shipyard_window_open_detour}
{ui_shipyard_window_open_detour}:
# eax and edx are already saved, and ecx is already set
call {shipyard_window_open_hook}
jmp [{continuation}]
",
ui_shipyard_window_open_detour = sym ui_shipyard_window_open_detour,
shipyard_window_open_hook = sym shipyard_window_open_hook,
continuation = sym SHIPYARD_WINDOW_OPEN_CONTINUATION);

global_asm!("
.global {load_shipyard_selected_page_detour}
{load_shipyard_selected_page_detour}:
# save regs
push ecx
push edx

call {shipyard_rendering_hook}

# restore regs
pop edx
pop ecx

jmp [{continuation}]
",
load_shipyard_selected_page_detour = sym load_shipyard_selected_page_detour,
shipyard_rendering_hook = sym shipyard_rendering_hook,
continuation = sym LOAD_SHIPYARD_SELECTED_PAGE_CONTINUATION);
