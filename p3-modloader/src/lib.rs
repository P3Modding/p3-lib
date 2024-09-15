use std::{fs, path::Path};

use log::{debug, error, info, LevelFilter};

pub(crate) mod ffi;

pub(crate) fn load() {
    debug!("Loading mods");
    let mods_path = Path::new("./mods");
    if !mods_path.exists() {
        fs::create_dir(mods_path).unwrap();
    }

    let entries = fs::read_dir(mods_path).unwrap();
}
