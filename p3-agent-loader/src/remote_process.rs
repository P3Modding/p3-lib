#![allow(clippy::missing_safety_doc)]
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{CloseHandle, GetLastError, HANDLE},
        System::{
            Diagnostics::Debug::ReadProcessMemory,
            Threading::{OpenProcess, PROCESS_ALL_ACCESS},
        },
    },
};

use crate::P3AgentLoaderError;

pub struct RemoteProcess {
    pub handle: HANDLE,
}

impl RemoteProcess {
    pub unsafe fn new(pid: u32) -> Result<RemoteProcess, P3AgentLoaderError> {
        let handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid).map_err(P3AgentLoaderError::OpenProcessFailed)?;
        Ok(RemoteProcess { handle })
    }

    pub unsafe fn read<T: Sized>(&self, address: u32) -> Result<T, P3AgentLoaderError> {
        let mut buf: Vec<u8> = vec![0; std::mem::size_of::<T>()];
        // TODO use lpnumberofbytesread
        if !ReadProcessMemory(self.handle, address as _, buf.as_mut_ptr() as _, buf.len(), None).as_bool() {
            return Err(P3AgentLoaderError::ReadProcessMemoryFailed(GetLastError()));
        }
        let t: T = unsafe { std::ptr::read(buf.as_ptr() as *const _) };
        Ok(t)
    }

    pub unsafe fn read_string(&self, address: u32) -> Result<String, P3AgentLoaderError> {
        let mut buf: [u8; 100] = self.read(address)?;
        buf[buf.len() - 1] = 0;

        PCSTR::from_raw(buf.as_ptr()).to_string().map_err(P3AgentLoaderError::ReadStringFailed)
    }
}

impl Drop for RemoteProcess {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}
