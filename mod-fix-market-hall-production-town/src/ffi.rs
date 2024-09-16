use log::{debug, error, LevelFilter};
use std::{arch::global_asm, ffi::c_void};
use windows::Win32::{
    Foundation::{GetLastError, WIN32_ERROR},
    System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS},
};
pub static TITLE: &str = concat!(" Weekly production (fixed)", "\0");
const WEEKLY_PRODUCTION_STRING_PTR: *mut *const u8 = 0x006A081C as _;
static SEVEN: f32 = 7.0;
static FTOL_ADDRESS: u32 = 0x00639C28;
static CONTINUATION_ADDRESS: u32 = 0x005DEACC;
static BARREL_PATCH_ADDRESS: u32 = 0x005DEA18;
static BUNDLE_PATCH_ADDRESS: u32 = 0x005DEA73;

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(LevelFilter::Trace);

    // Patch barrels
    let barrel_patch_address: u32 = (&barrel_ware_detour) as *const _ as _;
    let relative_barrel_patch_address: u32 = barrel_patch_address.wrapping_sub(BARREL_PATCH_ADDRESS) - 5; //TODO move pointer calcs to hooklet

    let mut barrel_patch: [u8; 5] = [0; 5];
    barrel_patch[0] = 0xe9;
    barrel_patch[1..5].copy_from_slice(&relative_barrel_patch_address.to_le_bytes());
    let patch_ptr: *mut u8 = BARREL_PATCH_ADDRESS as _;

    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(patch_ptr as _, 5, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed: {:?}", error);
        return 1;
    }

    debug!("Deploying barrel patch at {:x?}", barrel_patch_address as *const u32);
    patch_ptr.copy_from(barrel_patch.as_ptr(), barrel_patch.len());

    if !VirtualProtect(patch_ptr as _, 5, old_flags, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed: {:?}", error);
        return 2;
    }

    // Patch bundles
    let bundle_patch_address: u32 = (&bundle_ware_detour) as *const _ as _;
    let relative_bundle_patch_address: u32 = bundle_patch_address.wrapping_sub(BUNDLE_PATCH_ADDRESS) - 5; //TODO move pointer calcs to hooklet

    let mut bundle_patch: [u8; 5] = [0; 5];
    bundle_patch[0] = 0xe9;
    bundle_patch[1..5].copy_from_slice(&relative_bundle_patch_address.to_le_bytes());
    let patch_ptr: *mut u8 = BUNDLE_PATCH_ADDRESS as _;

    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(patch_ptr as _, 5, PAGE_EXECUTE_READWRITE, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect PAGE_EXECUTE_READWRITE failed: {:?}", error);
        return 1;
    }

    debug!("Deploying bundle patch at {:x?}", bundle_patch_address as *const u32);
    patch_ptr.copy_from(bundle_patch.as_ptr(), bundle_patch.len());

    if !VirtualProtect(patch_ptr as _, 5, old_flags, &mut old_flags).as_bool() {
        let error: WIN32_ERROR = GetLastError();
        error!("VirtualProtect restore failed: {:?}", error);
        return 2;
    }

    // Patch title
    *WEEKLY_PRODUCTION_STRING_PTR = TITLE.as_ptr();
    0
}

extern "C" {
    static barrel_ware_detour: c_void;
    static bundle_ware_detour: c_void;
}

// Barrel wares
global_asm!("
.global {barrel_ware_detour}
{barrel_ware_detour}:
shl    edx,0x6
sub    edx,eax
lea    ebp,[ecx+edx*2]
fild   DWORD PTR [esi+ebp*4+0x4]
fstp   DWORD PTR [esp+0x20]
fld    DWORD PTR [esp+0x20]
fdiv   DWORD PTR ds:0x66a91c
fadd   QWORD PTR ds:0x66a7d0
call   [{ftol_addresss}]
fild   DWORD PTR [esi+ebp*4+0xc4]
mov    edi,eax
fstp   DWORD PTR [esp+0x20]
fld    DWORD PTR [esp+0x20]
fdiv   DWORD PTR ds:0x66a91c
fmul   DWORD PTR ds:{seven}
fadd   QWORD PTR ds:0x66a7d0
call   [{ftol_addresss}]
fild   DWORD PTR [esp+0x18]
fstp   DWORD PTR [esp+0x20]
fld    DWORD PTR [esp+0x20]
fdiv   DWORD PTR ds:0x66a91c
jmp    [{continuation_address}]
",
barrel_ware_detour = sym barrel_ware_detour,
ftol_addresss = sym FTOL_ADDRESS,
seven = sym SEVEN,
continuation_address = sym CONTINUATION_ADDRESS);

// Bundle wares
global_asm!("
.global {bundle_ware_detour}
{bundle_ware_detour}:
shl    edx,0x6
sub    edx,eax
lea    ebp,[ecx+edx*2]
fild   DWORD PTR [esi+ebp*4+0x4]
fstp   DWORD PTR [esp+0x20]
fld    DWORD PTR [esp+0x20]
fdiv   DWORD PTR ds:0x66a918
fadd   QWORD PTR ds:0x66a7d0
call   [{ftol_addresss}]
fild   DWORD PTR [esi+ebp*4+0xc4]
mov    edi,eax
fstp   DWORD PTR [esp+0x20]
fld    DWORD PTR [esp+0x20]
fdiv   DWORD PTR ds:0x66a918
fmul   DWORD PTR ds:{seven}
fadd   QWORD PTR ds:0x66a7d0
call   [{ftol_addresss}]
fild   DWORD PTR [esp+0x18]
fstp   DWORD PTR [esp+0x20]
fld    DWORD PTR [esp+0x20]
fdiv   DWORD PTR ds:0x66a918 
jmp    [{continuation_address}]
",
bundle_ware_detour = sym bundle_ware_detour,
ftol_addresss = sym FTOL_ADDRESS,
seven = sym SEVEN,
continuation_address = sym CONTINUATION_ADDRESS);
