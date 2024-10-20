use log::debug;
use p3_api::{data::class48::Class48Ptr, ui::ui_town_hall_window::UITownHallWindowPtr};
use pages::{aldermans_office, details};

pub(crate) mod ffi;
pub(crate) mod pages;

pub(crate) unsafe fn handle_open() {
    debug!("handle_open");
    let class48 = Class48Ptr::new();
    class48.set_ignore_below_gradient(0);
    class48.set_gradient_y(0);
}

pub(crate) unsafe fn handle_set_selected_page(page: u32) {
    debug!("handle_set_selected_page");
    if page == 7 {
        let class48 = Class48Ptr::new();
        class48.set_ignore_below_gradient(0);
        class48.set_gradient_y(0);
    }
}

pub(crate) unsafe fn handle_selected_page_switch() -> i32 {
    let window = UITownHallWindowPtr::new();
    let selected_page = window.get_selected_page();

    if selected_page == -1 {
        details::draw_page(window);
        window.set_field_1930_timestamp(0);
    } else if selected_page == 7 {
        aldermans_office::draw_page(window);
        window.set_field_1930_timestamp(0);
    }

    selected_page
}
