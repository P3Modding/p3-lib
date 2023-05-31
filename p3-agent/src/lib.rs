use std::{ffi::c_void, net::TcpListener, sync::{Mutex, mpsc::{Sender, Receiver, channel}}, thread};

use ffi::_00535760_hook;
use log::{debug, trace, warn};
use server::run_server;
use windows::{
    core::PCSTR,
    s,
    Win32::System::{
        Diagnostics::Debug::{DebugBreak, OutputDebugStringA},
        Memory::{VirtualProtect, PAGE_EXECUTE, PAGE_EXECUTE_READWRITE, PAGE_READWRITE},
    },
};

pub mod ffi;
pub mod server;

static CONTEXT: Mutex<Option<Context>> = Mutex::new(None);

const HOOK1_ADDRESS: u32 = 0x00546935;
const HOOK1_ORIGINAL_VALUE: u32 = 0xfffeee27;

pub(crate) struct Context {
    pub(crate) sender: Sender<Vec<u8>>,
    pub(crate) receiver: Receiver<Vec<u8>>,
}

pub(crate) fn run() {
    let mut mg = CONTEXT.lock().unwrap();
    trace!("run()");

    if mg.is_none() {
        *mg = Some(Context::new());
    }
    let context: &mut Context = mg.as_mut().unwrap();
    match context.receiver.try_recv() {
        Ok(e) => {
            // Pass the command to P3
            unsafe { ffi::schedule_operation(&e) }
        },
        Err(_) => {},
    }
}

impl Context {
    pub fn new() -> Self {
        debug!("Context::new()");
        thread::spawn(|| run_server());
        let (sender, receiver) = channel();
        Context { sender, receiver }
    }
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(
    module: u32,
    reason_for_call: u32,
    resesrved: *mut c_void,
) {
    let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
    log::set_max_level(log::LevelFilter::Debug);
    debug!("DllMain({:#x}, {}, {})", module, reason_for_call, resesrved as u32);
    let ptr: *mut u32 = HOOK1_ADDRESS as _;
    match reason_for_call {
        1 => {
            // Hook
            let new_address = _00535760_hook as u32;
            let new_value = new_address.wrapping_sub(HOOK1_ADDRESS - 1 + 5); // -1 for E8, +5 for the size of the call
            debug!("Patching {:#x} to call {:#x}", HOOK1_ADDRESS, new_address);
            ptr.write_volatile(new_value);
        }
        0 => {
            debug!("PROCESS_DETACH");
            // TODO
            //*ptr = HOOK1_ORIGINAL_VALUE;
        }
        _ => {}
    }
}
