#![allow(clippy::missing_safety_doc)]
use log::{debug, error, info, trace};
use server::run_server;
use std::{
    sync::{
        atomic::{AtomicU8, Ordering},
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    thread::{self},
    time::Duration,
};
use windows::{
    imp::GetLastError,
    Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE},
};

use crate::ffi::{GAME_WORLD, P3};

pub mod ffi;
pub mod routes;
pub mod server;

static CONTEXT: Mutex<Option<AgentContext>> = Mutex::new(None);
static SERVER_STATUS: AtomicU8 = AtomicU8::new(0);

const HOOK1_ADDRESS: u32 = 0x00531036;
const HOOK1_ORIGINAL_VALUE: u32 = 0x000E1786;
const HOOK2_ADDRESS: u32 = 0x00519D73; // ship docked notification
const HOOK2_ORIGINAL_VALUE: u32 = 0x0002EF29;
const HOOK3_ADDRESS: u32 = 0x0050777D; // convoy docked notification
const HOOK3_ORIGINAL_VALUE: u32 = 0x0004151F;
const STATUS_RUNNING: u8 = 1;
const STATUS_SHUTDOWN: u8 = 2;
const STATUS_SHUTDOWN_FINISHED: u8 = 3;

pub(crate) struct AgentContext {
    pub(crate) sender: Sender<Vec<u8>>,
    pub(crate) receiver: Receiver<Vec<u8>>,
}

pub(crate) fn tick() {
    let time = GAME_WORLD.get_game_time_raw(&P3).unwrap();
    if time & 0b00001111 != 0b00001111 {
        return
    }
    let mut mg = CONTEXT.lock().unwrap();
    debug!("tick()");

    let context: &mut AgentContext = mg.as_mut().unwrap();
    if let Ok(command) = context.receiver.try_recv() {
        ffi::schedule_operation_raw(&command)
    }

    routes::tick_routes();

    trace!("tick() done");
}

impl AgentContext {
    pub fn new() -> Self {
        debug!("Context::new()");
        let _server_thread = thread::spawn(run_server);
        let (sender, receiver) = channel();
        AgentContext { sender, receiver }
    }
}

#[no_mangle]
pub unsafe extern "C" fn start() -> u32 {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Trace);
    debug!("start()");

    // Setup
    SERVER_STATUS.store(STATUS_RUNNING, Ordering::SeqCst);
    let mut mg = CONTEXT.lock().unwrap();
    *mg = Some(AgentContext::new());
    if let Err(e) = routes::init_routes() {
        error!("init_routes failed: {:?}", e);
        return 0;
    }

    // Hook 1
    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(HOOK1_ADDRESS as _, 4, PAGE_READWRITE, &mut old_flags).as_bool() {
        error!("VirtualProtect PAGE_READWRITE failed: {}", GetLastError());
        return 0;
    }

    let ptr: *mut u32 = HOOK1_ADDRESS as _;
    let new_address = ffi::handle_class11_tick_hook as usize as u32;
    let new_value = new_address.wrapping_sub(HOOK1_ADDRESS - 1 + 5); // -1 for E8, +5 for the size of the call
    debug!("Patching {:#x} to call {:#x}", HOOK1_ADDRESS, new_address);
    ptr.write_volatile(new_value);

    if !VirtualProtect(HOOK1_ADDRESS as _, 4, PAGE_EXECUTE, &mut old_flags).as_bool() {
        error!("VirtualProtect restore failed: {}", GetLastError());
        return 0;
    }

    // Hook 2
    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(HOOK2_ADDRESS as _, 4, PAGE_READWRITE, &mut old_flags).as_bool() {
        error!("VirtualProtect PAGE_READWRITE failed: {}", GetLastError());
        return 0;
    }

    let ptr: *mut u32 = HOOK2_ADDRESS as _;
    let new_address = ffi::handle_ship_docked_do_notification_hook as usize as u32;
    let new_value = new_address.wrapping_sub(HOOK2_ADDRESS - 1 + 5); // -1 for E8, +5 for the size of the call
    debug!("Patching {:#x} to call {:#x}", HOOK2_ADDRESS, new_address);
    ptr.write_volatile(new_value);

    if !VirtualProtect(HOOK2_ADDRESS as _, 4, PAGE_EXECUTE, &mut old_flags).as_bool() {
        error!("VirtualProtect restore failed: {}", GetLastError());
        return 0;
    }

    // Hook 3
    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(HOOK3_ADDRESS as _, 4, PAGE_READWRITE, &mut old_flags).as_bool() {
        error!("VirtualProtect PAGE_READWRITE failed: {}", GetLastError());
        return 0;
    }

    let ptr: *mut u32 = HOOK3_ADDRESS as _;
    let new_address = ffi::handle_ship_docked_do_notification_hook as usize as u32;
    let new_value = new_address.wrapping_sub(HOOK3_ADDRESS - 1 + 5); // -1 for E8, +5 for the size of the call
    debug!("Patching {:#x} to call {:#x}", HOOK3_ADDRESS, new_address);
    ptr.write_volatile(new_value);

    if !VirtualProtect(HOOK3_ADDRESS as _, 4, PAGE_EXECUTE, &mut old_flags).as_bool() {
        error!("VirtualProtect restore failed: {}", GetLastError());
        return 0;
    }

    info!("Start completed sucessfully");
    1
}

#[no_mangle]
pub unsafe extern "C" fn stop() -> u32 {
    debug!("stop()");
    // Remove Hook
    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(HOOK1_ADDRESS as _, 4, PAGE_READWRITE, &mut old_flags).as_bool() {
        error!("VirtualProtect PAGE_READWRITE failed: {}", GetLastError());
        return 0;
    }

    let ptr: *mut u32 = HOOK1_ADDRESS as _;
    debug!("Back-patching {:#x} to call {:#x} again", HOOK1_ADDRESS, HOOK1_ORIGINAL_VALUE);
    ptr.write_volatile(HOOK1_ORIGINAL_VALUE);

    if !VirtualProtect(HOOK1_ADDRESS as _, 4, PAGE_EXECUTE, &mut old_flags).as_bool() {
        error!("VirtualProtect restore failed: {}", GetLastError());
        return 0;
    }

    // Shutdown server
    debug!("Shutting down server");
    SERVER_STATUS.store(STATUS_SHUTDOWN, Ordering::SeqCst);
    loop {
        if SERVER_STATUS.load(Ordering::SeqCst) == STATUS_SHUTDOWN_FINISHED {
            break;
        }
        thread::sleep(Duration::from_millis(10))
    }

    info!("Stop completed sucessfully");
    1
}
