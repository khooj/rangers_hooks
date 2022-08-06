mod sigscan;
mod player;
mod main_thread;
mod handler;

use std::error::Error;
use std::ffi::{CString, c_void};
use std::mem;

use main_thread::MainThread;
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress, DisableThreadLibraryCalls};
use windows::Win32::System::{SystemInformation::GetSystemDirectoryA, SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}};
use windows::core::{PCSTR, HRESULT};
use windows::Win32::Foundation::{HINSTANCE, MAX_PATH, BOOL};

type LPVOID = *const c_void;
type DWORD = u32;

unsafe fn main() -> Result<(), Box<dyn Error>> {
    MainThread::start().expect("can't start main thread");

    Ok(())
}

unsafe fn uninit() {
    MainThread::stop().expect("can't stop thread");
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    module: HINSTANCE,
    call_reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        DisableThreadLibraryCalls(module);
        AllocConsole();
        let r = main();
        if let Err(v) = r {
            println!("error: {}", v);
            false.into()
        } else {
            true.into()
        }
    } else if call_reason == DLL_PROCESS_DETACH {
        uninit();
        true.into()
    } else {
        true.into()
    }
}