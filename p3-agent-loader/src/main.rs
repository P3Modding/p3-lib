use clap::Parser;
use log::{debug, error, info, LevelFilter};
use sysinfo::{PidExt, ProcessExt, System, SystemExt, Process};
use windows::{
    core::Error,
    imp::{GetLastError, GetProcAddress},
    s,
    Win32::{
        Foundation::CloseHandle,
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            LibraryLoader::GetModuleHandleA,
            Memory::{VirtualAllocEx, VirtualFreeEx, MEM_COMMIT, MEM_RELEASE, PAGE_READWRITE},
            Threading::{CreateRemoteThread, GetExitCodeThread, OpenProcess, WaitForSingleObject, INFINITE, PROCESS_ALL_ACCESS},
        },
    },
};

#[derive(clap::ValueEnum, Clone, Debug)]
enum Operation {
    Load,
    Unload,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Operation
    #[clap(value_enum, default_value_t=Operation::Load)]
    operation: Operation,
}

#[derive(Clone, Debug)]
pub enum P3AgentLoaderError {
    OpenProcessFailed(Error),
    GetModuleHandleAFailed(Error),
    GetProcAddressFailed(u32),
    VirtualAllocExFailed(u32),
    WriteProcessMemoryFailed(u32),
    CreateRemoteThreadFailed(Error),
    GetExitCodeThreadFailed,
    LoadLibraryAFailed,
    FreeLibraryFailed(u32),
}

fn main() {
    let _ = simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();
    let args = Args::parse();
    let s = System::new_all();
    let patricians: Vec<&Process> = s.processes_by_exact_name("Patrician3.exe").collect();
    if patricians.is_empty() {
        error!("Could not find Patrician3.exe");
        return;
    }

    match args.operation {
        Operation::Load => {
            for process in patricians {
                if let Err(e) = load(process.pid().as_u32()) {
                    error!("Load failed: {:?}", e);
                }
            }
        }
        Operation::Unload => {
            for process in patricians {
                if let Err(e) = unload(process.pid().as_u32(), 0x7462_0000) {
                    error!("Unload failed: {:?}", e);
                }
            }
        }
    }
}

fn load(pid: u32) -> Result<(), P3AgentLoaderError> {
    unsafe {
        let handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid).map_err(P3AgentLoaderError::OpenProcessFailed)?;
        debug!("P3 opened sucessfully");

        // Get handle of kernel32.dll
        let kernel_32 = match GetModuleHandleA(s!("Kernel32")) {
            Ok(m) => m,
            Err(e) => {
                CloseHandle(handle);
                return Err(P3AgentLoaderError::GetModuleHandleAFailed(e));
            }
        };

        // Get address of LoadLibrary
        let load_library_a_address = GetProcAddress(kernel_32.0, s!("LoadLibraryA"));
        if load_library_a_address.is_null() {
            CloseHandle(handle);
            return Err(P3AgentLoaderError::GetProcAddressFailed(GetLastError()));
        }
        debug!("LoadLibraryA is at {:#x}", load_library_a_address as usize);

        // Allocate LoadLibrary argument buffer
        let path = s!(r"C:\Users\Benni\repositories\p3-lib\target\i686-pc-windows-msvc\release\p3_agent.dll");
        let buf_ptr = VirtualAllocEx(handle, None, path.as_bytes().len(), MEM_COMMIT, PAGE_READWRITE);
        if buf_ptr.is_null() {
            CloseHandle(handle);
            return Err(P3AgentLoaderError::VirtualAllocExFailed(GetLastError()));
        }

        // Write dll path to buffer
        if !WriteProcessMemory(handle, buf_ptr, path.as_ptr() as _, path.as_bytes().len(), None).as_bool() {
            CloseHandle(handle);
            VirtualFreeEx(handle, buf_ptr, 4, MEM_RELEASE);
            return Err(P3AgentLoaderError::WriteProcessMemoryFailed(GetLastError()));
        }

        // Execute LoadLibraryA
        let thread = match CreateRemoteThread(handle, None, 0, Some(std::mem::transmute(load_library_a_address)), Some(buf_ptr), 0, None) {
            Ok(h) => h,
            Err(e) => {
                CloseHandle(handle);
                return Err(P3AgentLoaderError::CreateRemoteThreadFailed(e));
            }
        };

        debug!("Waiting for thread to finish...");
        WaitForSingleObject(thread, INFINITE);
        debug!("done!");

        // Check exit code
        let mut exit_code = 0;
        if !GetExitCodeThread(thread, &mut exit_code).as_bool() {
            CloseHandle(thread);
            VirtualFreeEx(handle, buf_ptr, 4, MEM_RELEASE);
            CloseHandle(handle);
            return Err(P3AgentLoaderError::GetExitCodeThreadFailed);
        }
        if exit_code == 0 {
            CloseHandle(thread);
            VirtualFreeEx(handle, buf_ptr, 4, MEM_RELEASE);
            CloseHandle(handle);
            return Err(P3AgentLoaderError::LoadLibraryAFailed);
        }

        info!("Module loaded sucessfully ({:x})", exit_code);
        CloseHandle(thread);
        VirtualFreeEx(handle, buf_ptr, 4, MEM_RELEASE);
        CloseHandle(handle);
        Ok(())
    }
}

fn unload(pid: u32, module_address: u32) -> Result<(), P3AgentLoaderError> {
    unsafe {
        let handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid).map_err(P3AgentLoaderError::OpenProcessFailed)?;
        debug!("P3 opened sucessfully");

        // Get handle of kernel32.dll
        let kernel_32 = match GetModuleHandleA(s!("Kernel32")) {
            Ok(m) => m,
            Err(e) => {
                CloseHandle(handle);
                return Err(P3AgentLoaderError::GetModuleHandleAFailed(e));
            }
        };

        // Get address of FreeLibrary
        let free_library_address = GetProcAddress(kernel_32.0, s!("FreeLibrary"));
        if free_library_address.is_null() {
            CloseHandle(handle);
            return Err(P3AgentLoaderError::GetProcAddressFailed(GetLastError()));
        }
        debug!("FreeLibrary is at {:#x}", free_library_address as usize);

        // Execute FreeLibrary
        let thread = match CreateRemoteThread(
            handle,
            None,
            0,
            Some(std::mem::transmute(free_library_address)),
            Some(module_address as *const _),
            0,
            None,
        ) {
            Ok(h) => h,
            Err(e) => {
                CloseHandle(handle);
                return Err(P3AgentLoaderError::CreateRemoteThreadFailed(e));
            }
        };

        debug!("Waiting for thread to finish...");
        WaitForSingleObject(thread, INFINITE);
        debug!("done!");

        // Check exit code
        let mut exit_code = 0;
        if !GetExitCodeThread(thread, &mut exit_code).as_bool() {
            CloseHandle(thread);
            CloseHandle(handle);
            return Err(P3AgentLoaderError::GetExitCodeThreadFailed);
        }
        if exit_code == 0 {
            CloseHandle(thread);
            CloseHandle(handle);
            return Err(P3AgentLoaderError::FreeLibraryFailed(exit_code));
        }

        info!("Module {:#x} freed sucessfully", module_address);
        CloseHandle(thread);
        CloseHandle(handle);
        Ok(())
    }
}
