#![allow(clippy::missing_safety_doc)]

pub(crate) mod commands;
mod handler;
mod main_thread;
mod player;
mod sigscan;
mod websockets;
mod world_data;

use std::error::Error;
use std::ffi::c_void;

use main_thread::MainThread;
use windows::Win32::Foundation::{BOOL, HINSTANCE};
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

type Lpvoid = *const c_void;
type Dword = u32;

unsafe fn main(module: HINSTANCE) -> Result<(), Box<dyn Error>> {
    MainThread::start(module).expect("can't start main thread");

    Ok(())
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    module: HINSTANCE,
    call_reason: Dword,
    _reserved: Lpvoid,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        DisableThreadLibraryCalls(module);
        AllocConsole();
        let r = main(module);
        if let Err(v) = r {
            println!("error: {}", v);
            false.into()
        } else {
            true.into()
        }
    } else if call_reason == DLL_PROCESS_DETACH {
        println!("started unloading");
        true.into()
    } else {
        true.into()
    }
}
