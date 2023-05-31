use std::{
    ffi::c_void,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Mutex,
    },
    thread,
};

use ffi::_00535760_hook;
use log::{debug, error, info, trace};
use server::run_server;
use windows::{
    imp::GetLastError,
    Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE},
};

pub mod ffi;
pub mod server;

static CONTEXT: Mutex<Option<Context>> = Mutex::new(None);

const HOOK1_ADDRESS: u32 = 0x00546935;
const _HOOK1_ORIGINAL_VALUE: u32 = 0xfffeee27;

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
    if let Ok(command) = context.receiver.try_recv() {
        ffi::schedule_operation(&command)
    }
}

impl Context {
    pub fn new() -> Self {
        debug!("Context::new()");
        thread::spawn(run_server);
        let (sender, receiver) = channel();
        Context { sender, receiver }
    }
}

#[no_mangle]
pub extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, resesrved: *mut c_void) {
    unsafe {
        let _ = log::set_logger(&win_dbg_logger::DEBUGGER_LOGGER);
        log::set_max_level(log::LevelFilter::Debug);
        debug!("DllMain({:#x}, {}, {})", module, reason_for_call, resesrved as u32);
        let ptr: *mut u32 = HOOK1_ADDRESS as _;
        match reason_for_call {
            1 => {
                // Hook
                let mut old_flags: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
                if !VirtualProtect(HOOK1_ADDRESS as _, 4, PAGE_READWRITE, &mut old_flags).as_bool() {
                    error!("VirtualProtect PAGE_READWRITE failed: {}", GetLastError());
                    return;
                }

                let new_address = _00535760_hook as usize as u32;
                let new_value = new_address.wrapping_sub(HOOK1_ADDRESS - 1 + 5); // -1 for E8, +5 for the size of the call
                debug!("Patching {:#x} to call {:#x}", HOOK1_ADDRESS, new_address);
                ptr.write_volatile(new_value);

                if !VirtualProtect(HOOK1_ADDRESS as _, 4, PAGE_EXECUTE, &mut old_flags).as_bool() {
                    error!("VirtualProtect restore failed: {}", GetLastError());
                    return;
                }

                info!("Hooks installed sucesfully");
            }
            0 => {
                debug!("PROCESS_DETACH");
                // TODO
                //*ptr = HOOK1_ORIGINAL_VALUE;
            }
            _ => {}
        }
    }
}
