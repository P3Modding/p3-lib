use std::{
    fs::{self},
    path::Path,
};

use log::{debug, error, info};
use windows::{
    core::{HSTRING, PCWSTR},
    s,
    Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW},
};

pub(crate) mod ffi;

pub(crate) fn load() {
    info!("Loading mods");
    let mods_path = Path::new("./mods");
    if !mods_path.exists() {
        fs::create_dir(mods_path).unwrap();
    }

    let mut n = 0;
    let entries = fs::read_dir(mods_path).unwrap();
    for entry in entries.flatten() {
        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        let filepath = format!("./mods/{}", entry.file_name().to_str().unwrap());
        info!("Loading {filepath}");
        unsafe {
            let filepath_hstring = HSTRING::from(filepath.clone());
            let hmodule = match LoadLibraryW(PCWSTR(filepath_hstring.as_ptr())) {
                Ok(hmodule) => hmodule,
                Err(e) => {
                    error!("Failed to LoadLibaryA {} ({:?}", entry.file_name().into_string().unwrap(), e);
                    continue;
                }
            };
            let start_address = match GetProcAddress(hmodule, s!("start")) {
                Some(start_address) => start_address,
                None => {
                    error!("Failed to GetProcAddress start for {}", entry.file_name().into_string().unwrap());
                    continue;
                }
            };

            debug!("Invoking start() for {} at {:#x}", filepath, start_address as usize);
            let start_result = start_address();
            if start_result != 0 {
                error!("Failed to start {} ({})", entry.file_name().into_string().unwrap(), start_result);
                continue;
            }

            info!("Loading of {filepath} succeeded");
            n += 1;
        }
    }

    info!("{n} mod(s) loaded successfully")
}
