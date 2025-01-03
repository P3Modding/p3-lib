use std::panic;

use log::error;

pub unsafe fn init_mod() {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    panic::set_hook(Box::new(|p| {
        error!("{p}");
    }));
}
