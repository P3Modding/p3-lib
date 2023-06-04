use std::string::FromUtf8Error;

use clap::Parser;
use log::{debug, error, info, LevelFilter};
use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};
use windows::{
    core::Error,
    s,
    Win32::{
        Foundation::{GetLastError, WIN32_ERROR},
        System::{
            Diagnostics::Debug::{IMAGE_DIRECTORY_ENTRY_EXPORT, IMAGE_NT_HEADERS32},
            LibraryLoader::{GetModuleHandleA, GetProcAddress},
            SystemServices::{IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY},
        },
    },
};

use crate::{remote_process::RemoteProcess, remote_thread::RemoteThread, remote_virtual_allocation::RemoteVirtualAllocation};

pub mod remote_process;
pub mod remote_thread;
pub mod remote_virtual_allocation;

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
    GetProcAddressFailed(WIN32_ERROR),
    VirtualAllocExFailed(WIN32_ERROR),
    WriteProcessMemoryFailed(WIN32_ERROR),
    ReadProcessMemoryFailed(WIN32_ERROR),
    ReadStringFailed(FromUtf8Error),
    CreateRemoteThreadFailed(Error),
    GetExitCodeThreadFailed,
    LoadLibraryAFailed,
    FreeLibraryFailed(u32),
    GetProcAddressRemoteFailed,
    ExportedFunctionNotFound,
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
                if let Err(e) = unload(process.pid().as_u32()) {
                    error!("Unload failed: {:?}", e);
                }
            }
        }
    }
}

fn load(pid: u32) -> Result<(), P3AgentLoaderError> {
    unsafe {
        let mut remote_process = RemoteProcess::new(pid)?;
        let kernel_32 = GetModuleHandleA(s!("Kernel32")).map_err(P3AgentLoaderError::GetModuleHandleAFailed)?;
        let load_library_a_address = GetProcAddress(kernel_32, s!("LoadLibraryA")).ok_or(P3AgentLoaderError::GetProcAddressFailed(GetLastError()))?;
        let path = s!(r"C:\Users\Benni\repositories\p3-lib\target\i686-pc-windows-msvc\release\p3_agent.dll");
        let mut remote_path = RemoteVirtualAllocation::new(&mut remote_process, path.as_bytes().len())?;
        remote_path.write(path.as_bytes())?;
        let p3_agent_module = RemoteThread::new(&mut remote_process, load_library_a_address as usize as _, Some(remote_path.ptr as _))?.wait()?;
        if p3_agent_module == 0 {
            return Err(P3AgentLoaderError::LoadLibraryAFailed);
        }

        run_exported_function(&mut remote_process, p3_agent_module, "start")?;

        info!("Module loaded sucessfully ({:x})", p3_agent_module);
        Ok(())
    }
}

fn unload(pid: u32) -> Result<(), P3AgentLoaderError> {
    unsafe {
        let mut remote_process = RemoteProcess::new(pid)?;
        let kernel_32 = GetModuleHandleA(s!("Kernel32")).map_err(P3AgentLoaderError::GetModuleHandleAFailed)?;
        let get_module_handle_address = GetProcAddress(kernel_32, s!("GetModuleHandleA")).ok_or(P3AgentLoaderError::GetProcAddressFailed(GetLastError()))?;
        let module_name = s!(r"p3_agent.dll");
        let mut buf_ptr = RemoteVirtualAllocation::new(&mut remote_process, module_name.as_bytes().len())?;
        buf_ptr.write(module_name.as_bytes())?;
        let p3_agent_module = RemoteThread::new(&mut remote_process, get_module_handle_address as usize as u32, Some(buf_ptr.ptr as _))?.wait()?;
        if p3_agent_module == 0 {
            return Err(P3AgentLoaderError::GetProcAddressRemoteFailed);
        }

        run_exported_function(&mut remote_process, p3_agent_module, "stop")?;

        let free_library_a_address = GetProcAddress(kernel_32, s!("FreeLibrary")).ok_or(P3AgentLoaderError::GetProcAddressFailed(GetLastError()))?;
        let exit_code = RemoteThread::new(&mut remote_process, free_library_a_address as usize as _, Some(p3_agent_module))?.wait()?;
        if exit_code == 0 {
            return Err(P3AgentLoaderError::FreeLibraryFailed(exit_code));
        }

        info!("Module {:#x} freed sucessfully", p3_agent_module);

        Ok(())
    }
}

fn run_exported_function(remote_process: &mut RemoteProcess, base_address: u32, function_name: &str) -> Result<u32, P3AgentLoaderError> {
    unsafe {
        // We'd love to call GetProcAddress to obtain the function pointer, but it requires 2 arguments and thus cannot be run with a simple CreateRemoteThread.
        // Instead, we'll go through the EAT, and acquire the function pointer there
        let dos_header: IMAGE_DOS_HEADER = remote_process.read(base_address)?;
        assert_eq!(dos_header.e_magic, 0x5a4d);
        let new_exe_header: IMAGE_NT_HEADERS32 = remote_process.read(base_address + dos_header.e_lfanew as usize as u32)?;
        assert_eq!(new_exe_header.Signature, 0x4550);
        let image_export_directory: IMAGE_EXPORT_DIRECTORY =
            remote_process.read(base_address + new_exe_header.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize].VirtualAddress)?;
        debug!("{:x?}", image_export_directory);

        let mut ordinal = None;
        for i in 0..image_export_directory.NumberOfNames {
            let name_offset: u32 = remote_process.read(base_address + image_export_directory.AddressOfNames + 4 * i)?;
            let name = remote_process.read_string(base_address + name_offset)?;
            if name == function_name {
                ordinal = Some(i);
                break;
            }
        }

        if let Some(ordinal) = ordinal {
            let function_offset: u32 = remote_process.read(base_address + image_export_directory.AddressOfFunctions + 4 * ordinal)?;
            debug!("function_offset={:x}", function_offset);
            RemoteThread::new(remote_process, base_address + function_offset, None)?.wait()
        } else {
            Err(P3AgentLoaderError::ExportedFunctionNotFound)
        }
    }
}
