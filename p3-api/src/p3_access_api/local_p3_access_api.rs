
use windows::{
    core::Error,
    Win32::{
        Foundation::HANDLE,
        System::Threading::{OpenProcess, PROCESS_VM_READ},
    },
};
use windows_sys::Win32::System::Diagnostics::Debug::ReadProcessMemory;

use super::P3AccessApi;

pub struct LocalP3AccessApi {
    handle: HANDLE,
}

impl P3AccessApi for LocalP3AccessApi {
    fn read_memory(&mut self, address: u32, dst: &mut [u8]) {
        unsafe {
            ReadProcessMemory(
                self.handle.0,
                address as _,
                dst.as_mut_ptr() as _,
                dst.len(),
                0 as _,
            );
        }
    }
}

/*
// ReadProcessMemory (
    hprocess : super::super::super::Foundation:: HANDLE ,
    lpbaseaddress : *const ::core::ffi::c_void ,
    lpbuffer : *mut ::core::ffi::c_void ,
    nsize : usize ,
    lpnumberofbytesread : *mut usize ) -> super::super::super::Foundation:: BOOL );
 */
impl LocalP3AccessApi {
    pub fn new(pid: u32) -> Result<Self, Error> {
        unsafe {
            let handle = OpenProcess(PROCESS_VM_READ, false, pid)?;
            Ok(Self { handle })
        }
    }
}
