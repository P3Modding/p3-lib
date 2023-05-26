use std::ffi::c_char;

use crate::raw_aim_file::RawAimFile;

extern "C" {
    #[link_name = "?AIM_INIT@@YAXPAUAIM_IMAGE@@@Z"]
    pub fn aim_init(aim_file: *mut RawAimFile);
    #[link_name = "?AIM_CONVERT_FILE@@YAHPAUAIM_IMAGE@@PBD@Z"]
    pub fn aim_convert_file(aim_file: *mut RawAimFile, path: *const c_char);
    #[link_name = "?AIM_FREE@@YAXPAUAIM_IMAGE@@@Z"]
    pub fn aim_free(aim_file: *mut RawAimFile);
}
