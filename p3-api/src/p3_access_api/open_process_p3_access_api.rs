use windows::{
    core::Error,
    Win32::{
        Foundation::HANDLE,
        System::Threading::{OpenProcess, PROCESS_VM_READ},
    },
};
use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;

use crate::P3ApiError;

use super::P3AccessApi;

pub struct OpenProcessP3AccessApi {
    handle: HANDLE,
}

impl OpenProcessP3AccessApi {
    pub fn new(pid: u32) -> Result<Self, Error> {
        unsafe {
            let handle = OpenProcess(PROCESS_VM_READ, false, pid)?;
            Ok(Self { handle })
        }
    }
}

impl P3AccessApi for OpenProcessP3AccessApi {
    fn read_memory(&mut self, address: u32, dst: &mut [u8]) -> Result<(), P3ApiError> {
        unsafe {
            let mut bytes_read: usize = 0;
            ReadProcessMemory(
                self.handle.0,
                address as _,
                dst.as_mut_ptr() as _,
                dst.len(),
                &mut bytes_read as _,
            );
            if bytes_read != dst.len() {
                Err(P3ApiError::ReadError)
            } else {
                Ok(())
            }
        }
    }
}
