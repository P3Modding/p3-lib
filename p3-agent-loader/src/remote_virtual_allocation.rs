#![allow(clippy::missing_safety_doc)]
use std::ffi::c_void;

use log::debug;
use windows::Win32::{
    Foundation::{GetLastError, HANDLE},
    System::{
        Diagnostics::Debug::WriteProcessMemory,
        Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE},
    },
};

use crate::{remote_process::RemoteProcess, P3AgentLoaderError};

pub struct RemoteVirtualAllocation {
    pub handle: HANDLE,
    pub len: usize,
    pub ptr: *mut c_void,
}

impl RemoteVirtualAllocation {
    pub unsafe fn new(handle: &RemoteProcess, len: usize) -> Result<RemoteVirtualAllocation, P3AgentLoaderError> {
        debug!("VirtualAllocEx");
        let ptr = VirtualAllocEx(handle.handle, None, len, MEM_COMMIT, PAGE_READWRITE);
        if ptr.is_null() {
            return Err(P3AgentLoaderError::VirtualAllocExFailed(GetLastError()));
        }
        Ok(RemoteVirtualAllocation {
            handle: handle.handle,
            ptr,
            len,
        })
    }

    pub unsafe fn write(&mut self, bytes: &[u8]) -> Result<(), P3AgentLoaderError> {
        assert!(bytes.len() <= self.len);
        if !WriteProcessMemory(self.handle, self.ptr, bytes.as_ptr() as _, bytes.len(), None).as_bool() {
            return Err(P3AgentLoaderError::WriteProcessMemoryFailed(GetLastError()));
        }

        Ok(())
    }
}

impl Drop for RemoteVirtualAllocation {
    fn drop(&mut self) {
        unsafe {
            VirtualFreeEx(self.handle, self.ptr, self.len, MEM_RELEASE);
        }
    }
}
