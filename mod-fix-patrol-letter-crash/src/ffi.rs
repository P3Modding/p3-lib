//! Fixes the vanilla crash-to-desktop when the personal-letters list shows a
//! scripted letter whose town byte is not a town (the "patrol mission" crash).
//!
//! The letter-creation script command (handler 0x4ed4a0) copies the low byte of a
//! script variable into the message's town byte (0x4ed4e4) without validation, and
//! the patrol/escort letter scripts pass a variable that is not a town index -
//! observed values include 40, 95, 228, 242 and 255. The letters list draws the
//! town column by indexing the 40-slot town-name bank with that byte, unbounded
//! (0x47d928: `mov eax, [edx*4+0x6dda00]`), and hands the result to ddraw_Dll's
//! text draw, which dereferences it without any guard (ddraw_Dll+0xf100). An
//! out-of-range byte therefore reads whatever UI global happens to live past the
//! bank: when that value aliases readable memory the row draws garbage or nothing,
//! and when it does not, the game dies on the spot. The letter body and header are
//! unaffected (they are formatted at creation through a bounded town-name helper).
//!
//! The fix detours the lookup: in-range ids read the bank exactly as before,
//! out-of-range ids draw an empty string - the same blank town column the
//! unpatched game shows on the days it happens to survive the read.

use log::{debug, error};
use std::arch::global_asm;
use std::ffi::c_void;
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
};

/// The row draw's unbounded title lookup, `mov eax,[edx*4+0x6dda00]` - 7 bytes,
/// replaced by `jmp title_lookup_detour` plus two NOPs.
const PATCH_ADDRESS: u32 = 0x0047d928;
const PATCH_LEN: usize = 7;
/// The bytes expected at [PATCH_ADDRESS]; refuse to patch anything else.
const ORIGINAL_BYTES: [u8; PATCH_LEN] = [0x8b, 0x04, 0x95, 0x00, 0xda, 0x6d, 0x00];
/// The instruction following the replaced one, where the detour jumps back to.
static CONTINUATION_ADDRESS: u32 = 0x0047d92f;

extern "C" {
    static title_lookup_detour: c_void;
}

/// What an out-of-range title draws. The game's text draw stops at the first NUL,
/// so pointing it here renders nothing.
static EMPTY_TITLE: u8 = 0;

// edx holds the title id, zero-extended by the game (xor edx,edx; mov dx,[edi-4]),
// and eax is the instruction's own output, so both are free to use. The town-name
// bank at 0x6dda00 has 40 slots.
global_asm!("
.global {detour}
{detour}:
cmp edx, 40
jae 2f
mov eax, dword ptr [edx*4 + 0x6dda00]
jmp [{continuation}]
2:
lea eax, [{empty}]
jmp [{continuation}]
",
    detour = sym title_lookup_detour,
    continuation = sym CONTINUATION_ADDRESS,
    empty = sym EMPTY_TITLE,
);

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    let original: [u8; PATCH_LEN] = *(PATCH_ADDRESS as *const [u8; PATCH_LEN]);
    if original != ORIGINAL_BYTES {
        error!("unexpected bytes at {PATCH_ADDRESS:#x}: {original:02x?} - wrong exe version?");
        return 3;
    }

    let detour_address = &title_lookup_detour as *const _ as u32;
    let mut patch = [0x90u8; PATCH_LEN];
    patch[0] = 0xe9;
    patch[1..5].copy_from_slice(&detour_address.wrapping_sub(PATCH_ADDRESS + 5).to_le_bytes());

    let patch_ptr: *mut u8 = PATCH_ADDRESS as _;
    let mut old_flags = PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(patch_ptr as _, PATCH_LEN, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed: {:?}", error);
        return 1;
    }

    debug!("Bounding the letter-title town lookup at {PATCH_ADDRESS:#x}");
    patch_ptr.copy_from(patch.as_ptr(), patch.len());

    if !VirtualProtect(patch_ptr as _, PATCH_LEN, old_flags, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed: {:?}", error);
        return 2;
    }

    0
}
