use log::{debug, error, LevelFilter};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use windows::{
    imp::{GetLastError, GetProcAddress},
    s,
    Win32::{
        Foundation::CloseHandle,
        System::{
            Diagnostics::Debug::WriteProcessMemory,
            LibraryLoader::GetModuleHandleA,
            Memory::{VirtualAllocEx, VirtualFreeEx, VirtualProtectEx, MEM_COMMIT, MEM_RELEASE, PAGE_PROTECTION_FLAGS, PAGE_READWRITE},
            Threading::{CreateRemoteThread, OpenProcess, WaitForSingleObject, INFINITE, PROCESS_ALL_ACCESS},
        },
    },
};

fn main() {
    let _ = simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();

    let s = System::new_all();
    for process in s.processes_by_name("Patrician") {
        load(process.pid().as_u32())
    }
}

fn load(pid: u32) {
    unsafe {
        let handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid).unwrap();
        debug!("P3 opened sucessfully");
        let mut old: PAGE_PROTECTION_FLAGS = windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS(0);
        if !VirtualProtectEx(handle, 0x00546935 as _, 4, PAGE_READWRITE, &mut old).as_bool() {
            error!("VirtualProtectEx failed: {}", GetLastError());
            return;
        }
        let path = s!(r"C:\Users\Benni\repositories\p3-lib\target\i686-pc-windows-msvc\release\p3_agent.dll");
        let buf_ptr = VirtualAllocEx(handle, None, path.as_bytes().len(), MEM_COMMIT, PAGE_READWRITE);
        debug!("Allocated buffer at {:#x}", buf_ptr as usize);
        assert!(WriteProcessMemory(handle, buf_ptr, path.as_ptr() as _, path.as_bytes().len(), None).0 != 0);
        debug!("Path to dll written to buffer sucessfully");
        let kernel_32 = GetModuleHandleA(s!("Kernel32")).unwrap();
        let load_library_a_address = GetProcAddress(kernel_32.0, s!("LoadLibraryA"));
        debug!("LoadLibrary is at {:#x}", load_library_a_address as usize);
        let thread = CreateRemoteThread(handle, None, 0, Some(std::mem::transmute(load_library_a_address)), Some(buf_ptr), 0, None).unwrap();
        debug!("Waiting for thread to finish...");
        WaitForSingleObject(thread, INFINITE);
        debug!("done!");
        assert!(CloseHandle(thread).0 != 0);
        debug!("Thread handle closed");
        if VirtualFreeEx(handle, buf_ptr, path.as_bytes().len(), MEM_RELEASE).0 != 0 {
            error!("VirtualFreeEx failed: {}", GetLastError());
            return;
        }
        debug!("Buffer freed sucessfully");
        assert!(CloseHandle(handle).0 != 0);
        debug!("P3 handle closed");
        //CreateRemoteThread(todo!(), todo!(), todo!(), todo!(), todo!(), todo!(), todo!());
    }
}

fn _unload(_pid: u32) {
    /*

    hThread = ::CreateRemoteThread( hProcess, NULL, 0,
                         (LPTHREAD_START_ROUTINE )::GetProcAddress(
                         ::GetModuleHandle(“Kernel32”), “FreeLibrary”),
                         (void*)hLibModule,
                          0, NULL );

    ::WaitForSingleObject( hThread, INFINITE );
    // Clean up
    ::CloseHandle( hThread );
        */
}
