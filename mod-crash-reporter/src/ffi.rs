//! Crash reporter: writes a full report to `_crash_report.txt` in the game
//! directory whenever the game raises a fatal-severity exception.
//!
//! The game dies to desktop without a dump, a WER dialog or DebugView output, so
//! crashes are captured first-chance: a vectored exception handler registered at
//! the front of the chain sees every exception before any of the game's SEH
//! handlers run, and appends a report - registers, the code bytes at EIP, the
//! memory around each register value, the EBP call chain and a scan of the stack
//! for return addresses (all attributed as module+offset) - before anything can
//! swallow the exception or kill the process. Only fatal-severity codes are
//! reported (C++ throws and debug prints are routine); a report the process
//! survives was a handled exception, so the last report in the file is the crash.
//! The unhandled-exception filter additionally marks reports that definitely
//! killed the process, when the game's own filter does not preempt it.
//!
//! Known blind spot: heap-corruption kills via fail-fast (`int 29`) never enter
//! exception dispatch, so a crash that leaves no report here points at the heap
//! (next step: PageHeap via `gflags /p /enable Patrician3.exe /full`, which turns
//! the corruption into an access violation this reporter catches).
//!
//! This mod found the patrol-letter crash fixed by mod-fix-patrol-letter-crash.

use std::fmt::Write as _;
use std::io::Write as _;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use windows::Win32::{
    Foundation::HMODULE,
    System::{
        Diagnostics::Debug::{AddVectoredExceptionHandler, SetUnhandledExceptionFilter, CONTEXT, EXCEPTION_POINTERS, EXCEPTION_RECORD},
        LibraryLoader::GetModuleFileNameA,
        Memory::{
            VirtualQuery, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_IMAGE, PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
            PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS,
        },
        Threading::GetCurrentThreadId,
    },
};

/// Reports are appended here, relative to the game's working directory (next to
/// Patrician3.exe, like the save folder).
const REPORT_PATH: &str = "_crash_report.txt";
/// The MSVC C++ throw code: raised and caught routinely, not a crash.
const MSVC_CPP_EXCEPTION: u32 = 0xE06D7363;
const EXCEPTION_CONTINUE_SEARCH: i32 = 0;

static IN_HANDLER: AtomicBool = AtomicBool::new(false);
static REPORT_COUNT: AtomicU32 = AtomicU32::new(0);

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);

    // First-position VEH: runs before every frame-based handler, so the report is
    // written even if something swallows the exception or the process dies without
    // reaching the unhandled filter.
    AddVectoredExceptionHandler(1, Some(vectored_handler));
    SetUnhandledExceptionFilter(Some(unhandled_filter));
    win_dbg_logger::output_debug_string("crash_reporter: installed\r\n");
    0
}

unsafe extern "system" fn vectored_handler(info: *mut EXCEPTION_POINTERS) -> i32 {
    report(info as *const EXCEPTION_POINTERS, "first-chance");
    EXCEPTION_CONTINUE_SEARCH
}

unsafe extern "system" fn unhandled_filter(info: *const EXCEPTION_POINTERS) -> i32 {
    report(info, "UNHANDLED - process is going down");
    EXCEPTION_CONTINUE_SEARCH
}

unsafe fn report(info: *const EXCEPTION_POINTERS, kind: &str) {
    let Some(info) = info.as_ref() else { return };
    let Some(record) = info.ExceptionRecord.as_ref() else { return };
    let code = record.ExceptionCode.0 as u32;
    // Fatal severity only (top two bits set): skips C++ throws, DBG_PRINTEXCEPTION
    // from OutputDebugString, breakpoints and other routine dispatch traffic.
    if code >> 30 != 3 || code == MSVC_CPP_EXCEPTION {
        return;
    }
    if IN_HANDLER.swap(true, Ordering::SeqCst) {
        // An exception inside the handler itself (or a concurrent one): bail rather
        // than recurse.
        return;
    }
    write_report(info, record, code, kind);
    IN_HANDLER.store(false, Ordering::SeqCst);
}

unsafe fn write_report(info: &EXCEPTION_POINTERS, record: &EXCEPTION_RECORD, code: u32, kind: &str) {
    let n = REPORT_COUNT.fetch_add(1, Ordering::SeqCst);
    let unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let mut out = String::with_capacity(8192);
    let _ = writeln!(
        out,
        "=== report {n}: {kind} {code:#010x} {} [unix {unix}, thread {:#x}] ===",
        code_name(code),
        GetCurrentThreadId()
    );
    let _ = writeln!(out, "at {}", describe_address(record.ExceptionAddress as u32));
    if code == 0xC0000005 && record.NumberParameters >= 2 {
        let op = match record.ExceptionInformation[0] {
            0 => "read",
            1 => "write",
            8 => "execute",
            _ => "access",
        };
        let _ = writeln!(out, "{op} of address {:#010x}", record.ExceptionInformation[1]);
    }

    if let Some(ctx) = info.ContextRecord.as_ref() {
        let _ = writeln!(
            out,
            "eax={:08x} ebx={:08x} ecx={:08x} edx={:08x} esi={:08x} edi={:08x}",
            ctx.Eax, ctx.Ebx, ctx.Ecx, ctx.Edx, ctx.Esi, ctx.Edi
        );
        let _ = writeln!(
            out,
            "eip={:08x} ebp={:08x} esp={:08x} eflags={:08x}",
            ctx.Eip, ctx.Ebp, ctx.Esp, ctx.EFlags
        );
        dump_code(&mut out, ctx.Eip);
        dump_register_memory(&mut out, ctx);
        walk_ebp_chain(&mut out, ctx);
        scan_stack(&mut out, ctx.Esp);
    } else {
        let _ = writeln!(out, "(no context record)");
    }
    let _ = writeln!(out, "=== end report {n} ===");

    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(REPORT_PATH) {
        let _ = f.write_all(out.as_bytes());
        let _ = f.flush();
    }
    win_dbg_logger::output_debug_string(&format!(
        "crash_reporter: report {n} written: {kind} {code:#010x} at {:#010x}\r\n",
        record.ExceptionAddress as u32
    ));
}

/// The code bytes around EIP, so the faulting instruction is identifiable even if the
/// address is not in a module (jumps through corrupted pointers land anywhere).
unsafe fn dump_code(out: &mut String, eip: u32) {
    let _ = writeln!(out, "code around eip:");
    let start = eip.saturating_sub(0x20) & !0xF;
    for row in 0..6u32 {
        let addr = start + row * 16;
        // Rows are 16-aligned and regions page-aligned, so one check covers the row.
        if !is_readable(addr) {
            let _ = writeln!(out, "  {addr:08x}: ??");
            continue;
        }
        let _ = write!(out, "  {addr:08x}:");
        for i in 0..16 {
            let b = core::ptr::read_unaligned((addr + i) as *const u8);
            let marker = if addr + i == eip { '>' } else { ' ' };
            let _ = write!(out, "{marker}{b:02x}");
        }
        let _ = writeln!(out);
    }
}

/// 0x40 bytes around each register that points into readable memory: the crash-time
/// contents of whatever structures the faulting code was working on.
unsafe fn dump_register_memory(out: &mut String, ctx: &CONTEXT) {
    let regs = [
        ("eax", ctx.Eax),
        ("ebx", ctx.Ebx),
        ("ecx", ctx.Ecx),
        ("edx", ctx.Edx),
        ("esi", ctx.Esi),
        ("edi", ctx.Edi),
        ("ebp", ctx.Ebp),
    ];
    for (name, value) in regs {
        let start = (value & !0xf).saturating_sub(0x10);
        // Both-ends check suffices: regions are page-aligned, so a 0x40 window spans
        // at most one boundary.
        if !is_readable(start) || !is_readable(start + 0x3f) {
            continue;
        }
        let _ = writeln!(out, "memory at {name} ({value:#010x}):");
        for row in 0..4u32 {
            let addr = start + row * 16;
            let bytes: Vec<String> = (0..16).map(|i| format!("{:02x}", core::ptr::read((addr + i) as *const u8))).collect();
            let _ = writeln!(out, "  {addr:08x}: {}", bytes.join(" "));
        }
    }
}

/// The classic x86 frame walk. MSVC-era code keeps frame pointers in most functions;
/// where a frame omits them the chain just ends early and the stack scan takes over.
unsafe fn walk_ebp_chain(out: &mut String, ctx: &CONTEXT) {
    let _ = writeln!(out, "ebp chain:");
    let mut ebp = ctx.Ebp;
    for i in 0..32 {
        let (Some(next), Some(ret)) = (read_u32(ebp), read_u32(ebp + 4)) else {
            let _ = writeln!(out, "  #{i} ebp={ebp:08x} (unreadable)");
            break;
        };
        let _ = writeln!(out, "  #{i} ebp={ebp:08x} ret={}", describe_address(ret));
        // Frames must ascend and stay plausibly close, or the chain is garbage.
        if ret == 0 || next <= ebp || next - ebp > 0x0010_0000 {
            break;
        }
        ebp = next;
    }
}

/// Every dword on the stack that points into module code is a return-address
/// candidate: recovers the call history even through frames without frame pointers,
/// at the price of some stale entries.
unsafe fn scan_stack(out: &mut String, esp: u32) {
    let _ = writeln!(out, "stack scan (dwords at esp pointing into module code):");
    for off in (0..0x400u32).step_by(4) {
        let Some(value) = read_u32(esp + off) else { break };
        if is_module_code(value) {
            let _ = writeln!(out, "  esp+{off:#05x}: {}", describe_address(value));
        }
    }
}

unsafe fn describe_address(addr: u32) -> String {
    match module_of(addr) {
        Some((name, base)) => format!("{addr:#010x} ({name}+{:#x})", addr - base),
        None => format!("{addr:#010x}"),
    }
}

/// The module (mapped image) containing `addr`, as (basename, load base).
unsafe fn module_of(addr: u32) -> Option<(String, u32)> {
    let mbi = query(addr)?;
    if mbi.State != MEM_COMMIT || mbi.Type != MEM_IMAGE {
        return None;
    }
    let base = mbi.AllocationBase as u32;
    let mut buf = [0u8; 260];
    let len = GetModuleFileNameA(HMODULE(base as isize), &mut buf) as usize;
    if len == 0 || len >= buf.len() {
        return None;
    }
    let path = String::from_utf8_lossy(&buf[..len]).into_owned();
    let name = path.rsplit(['\\', '/']).next().unwrap_or(&path).to_string();
    Some((name, base))
}

unsafe fn query(addr: u32) -> Option<MEMORY_BASIC_INFORMATION> {
    let mut mbi: MEMORY_BASIC_INFORMATION = core::mem::zeroed();
    if VirtualQuery(Some(addr as *const core::ffi::c_void), &mut mbi, core::mem::size_of::<MEMORY_BASIC_INFORMATION>()) == 0 {
        return None;
    }
    Some(mbi)
}

unsafe fn is_readable(addr: u32) -> bool {
    query(addr).is_some_and(|mbi| {
        mbi.State == MEM_COMMIT && mbi.Protect.0 != 0 && mbi.Protect.0 & (PAGE_NOACCESS.0 | PAGE_GUARD.0) == 0
    })
}

unsafe fn is_module_code(addr: u32) -> bool {
    query(addr).is_some_and(|mbi| {
        mbi.State == MEM_COMMIT
            && mbi.Type == MEM_IMAGE
            && mbi.Protect.0 & (PAGE_EXECUTE.0 | PAGE_EXECUTE_READ.0 | PAGE_EXECUTE_READWRITE.0 | PAGE_EXECUTE_WRITECOPY.0) != 0
    })
}

unsafe fn read_u32(addr: u32) -> Option<u32> {
    // The dword must not cross out of its (page-aligned) region.
    let mbi = query(addr)?;
    let region_end = mbi.BaseAddress as u32 + mbi.RegionSize as u32;
    if mbi.State != MEM_COMMIT || mbi.Protect.0 == 0 || mbi.Protect.0 & (PAGE_NOACCESS.0 | PAGE_GUARD.0) != 0 || addr > region_end - 4 {
        return None;
    }
    Some(core::ptr::read_unaligned(addr as *const u32))
}

fn code_name(code: u32) -> &'static str {
    match code {
        0xC0000005 => "ACCESS_VIOLATION",
        0xC000001D => "ILLEGAL_INSTRUCTION",
        0xC0000025 => "NONCONTINUABLE_EXCEPTION",
        0xC000008C => "ARRAY_BOUNDS_EXCEEDED",
        0xC000008E => "FLOAT_DIVIDE_BY_ZERO",
        0xC0000090 => "FLOAT_INVALID_OPERATION",
        0xC0000094 => "INTEGER_DIVIDE_BY_ZERO",
        0xC0000095 => "INTEGER_OVERFLOW",
        0xC0000096 => "PRIVILEGED_INSTRUCTION",
        0xC00000FD => "STACK_OVERFLOW",
        0xC0000374 => "HEAP_CORRUPTION",
        0xC0000409 => "FAIL_FAST/STACK_BUFFER_OVERRUN",
        0xC0000417 => "INVALID_CRUNTIME_PARAMETER",
        _ => "",
    }
}
