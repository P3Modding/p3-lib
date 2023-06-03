#![allow(clippy::missing_safety_doc)]
use std::mem;

use log::debug;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Threading::{CreateRemoteThread, GetExitCodeThread, WaitForSingleObject, INFINITE},
};

use crate::{remote_process::RemoteProcess, P3AgentLoaderError};

pub struct RemoteThread {
    pub handle: HANDLE,
}

impl RemoteThread {
    pub unsafe fn new(remote_process: &mut RemoteProcess, start_address: u32, arg: Option<u32>) -> Result<RemoteThread, P3AgentLoaderError> {
        let thread = CreateRemoteThread(remote_process.handle, None, 0, mem::transmute(start_address), arg.map(|u| u as _), 0, None)
            .map_err(P3AgentLoaderError::CreateRemoteThreadFailed)?;

        Ok(RemoteThread { handle: thread })
    }

    pub unsafe fn wait(self) -> Result<u32, P3AgentLoaderError> {
        debug!("Waiting for thread to finish...");
        WaitForSingleObject(self.handle, INFINITE);
        debug!("done!");

        // Check exit code
        let mut exit_code = 0;
        if !GetExitCodeThread(self.handle, &mut exit_code).as_bool() {
            return Err(P3AgentLoaderError::GetExitCodeThreadFailed);
        }

        Ok(exit_code)
    }
}

impl Drop for RemoteThread {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}
