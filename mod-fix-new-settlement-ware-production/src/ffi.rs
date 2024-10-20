use hooklet::windows::x86::replace_slice_rwx;
use log::{debug, error};

const SUBTRACTION_PATCH_ADDRESS: u32 = 0x00532FF3;
const WHALEOIL_PATCH_ADDRESS: u32 = 0x00533013;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    debug!("Patching `sub` immediate value at {:#x}", SUBTRACTION_PATCH_ADDRESS);
    let subtraction_patch = [4];
    match replace_slice_rwx(SUBTRACTION_PATCH_ADDRESS, &subtraction_patch) {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to deploy patch: {:?}", e);
            return 1;
        }
    }

    debug!("Patching whaleoil `or` immediate value at {:#x}", SUBTRACTION_PATCH_ADDRESS);
    let whaleoil_patch = 0x20002u32.to_le_bytes();
    match replace_slice_rwx(WHALEOIL_PATCH_ADDRESS, &whaleoil_patch) {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to deploy patch: {:?}", e);
            return 2;
        }
    }

    0
}
