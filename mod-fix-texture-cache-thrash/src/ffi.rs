use log::{error, info};
use windows::core::s;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

/// ddraw_dll.dll (the game's SGL graphics library) keeps decoded sprite images in an
/// LRU cache: usage counter at module+0x80F14, eviction loops at +0x1C2EF/+0x1C39A
/// that release least-recently-used images while the usage exceeds the budget at
/// module+0x5F734. The budget's built-in default is 16 MiB (0x01000000), but one town
/// view plus an open building window needs ~19.4 MB - slightly more than the budget -
/// so the evictor removes exactly the images the next frame redraws, and the whole
/// dimmed scene plus the building interior re-decodes from the archives EVERY FRAME
/// (measured: ~800 ms of AIM.dll decoding per second, 48 -> 17 fps; worst in the
/// shipyard, whose animated overlays make the working set largest).
///
/// GOG ships gl.cfg with "TextureCacheSize = 48000000", which would be plenty - but
/// this build of ddraw_dll never reads it: the gl.cfg section parser (+0x12190) has
/// no callers and nothing references the "gl.cfg" filename string. Dead code.
///
/// The fix is this one dword: raise the budget to 48 MiB - about 2.5x the measured
/// working set, and the same ballpark as the 48000000 GOG tried to configure. The
/// counter only ever grows to what is actually in use, so real memory use rises by a
/// few dozen MB at most. Bigger is not better: a display mode switch (alt+tab, or
/// entering the menu at its own resolution) releases and rebuilds the cached surfaces,
/// so a larger cache makes those switches slower. The value is patched only if the
/// default is found, so a different ddraw_dll build is left alone.
const CACHE_LIMIT_RVA: u32 = 0x5F734;
const DEFAULT_LIMIT: u32 = 0x0100_0000;
const RAISED_LIMIT: u32 = 0x0300_0000;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    // ddraw_dll.dll may not be loaded yet while the modloader runs start(); poll from
    // a thread and patch once it appears. Nothing else ever writes the budget (the
    // only writer would be the dead gl.cfg parser), so a one-time store is safe; the
    // aligned u32 store is atomic on x86.
    std::thread::spawn(|| unsafe { patch_when_loaded() });
    0
}

unsafe fn patch_when_loaded() {
    for _ in 0..240 {
        if let Ok(module) = GetModuleHandleA(s!("ddraw_dll.dll")) {
            if !module.is_invalid() {
                let limit = (module.0 as u32).wrapping_add(CACHE_LIMIT_RVA) as *mut u32;
                let current = *limit;
                if current == DEFAULT_LIMIT {
                    *limit = RAISED_LIMIT;
                    info!("texture cache budget raised {DEFAULT_LIMIT:#010x} -> {RAISED_LIMIT:#010x}");
                } else if current == RAISED_LIMIT {
                    info!("texture cache budget already raised");
                } else {
                    error!("unexpected texture cache budget {current:#010x} - not patching (different ddraw_dll build?)");
                }
                return;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    error!("ddraw_dll.dll never loaded - texture cache budget not raised");
}
