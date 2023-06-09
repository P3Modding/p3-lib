#![allow(clippy::missing_safety_doc)]
use ffi::_00535760_hook;
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

pub mod ffi;
pub mod routes;
pub mod server;

static CONTEXT: Mutex<Option<AgentContext>> = Mutex::new(None);
static SERVER_STATUS: AtomicU8 = AtomicU8::new(0);

const HOOK1_ADDRESS: u32 = 0x00546935;
const HOOK1_ORIGINAL_VALUE: u32 = 0xfffeee27;
const STATUS_RUNNING: u8 = 1;
const STATUS_SHUTDOWN: u8 = 2;
const STATUS_SHUTDOWN_FINISHED: u8 = 3;

pub(crate) struct AgentContext {
    pub(crate) sender: Sender<Vec<u8>>,
    pub(crate) receiver: Receiver<Vec<u8>>,
}

pub(crate) fn tick() {
    let mut mg = CONTEXT.lock().unwrap();
    trace!("tick()");

    let context: &mut AgentContext = mg.as_mut().unwrap();
    if let Ok(command) = context.receiver.try_recv() {
        ffi::schedule_operation_raw(&command)
    }

    routes::tick_routes();

    trace!("run done");
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

    // Hook
    let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
    if !VirtualProtect(HOOK1_ADDRESS as _, 4, PAGE_READWRITE, &mut old_flags).as_bool() {
        error!("VirtualProtect PAGE_READWRITE failed: {}", GetLastError());
        return 0;
    }

    let ptr: *mut u32 = HOOK1_ADDRESS as _;
    let new_address = _00535760_hook as usize as u32;
    let new_value = new_address.wrapping_sub(HOOK1_ADDRESS - 1 + 5); // -1 for E8, +5 for the size of the call
    debug!("Patching {:#x} to call {:#x}", HOOK1_ADDRESS, new_address);
    ptr.write_volatile(new_value);

    if !VirtualProtect(HOOK1_ADDRESS as _, 4, PAGE_EXECUTE, &mut old_flags).as_bool() {
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
