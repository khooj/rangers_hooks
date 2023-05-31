use std::ffi::c_void;
use windows::Win32::{
    Foundation::{CloseHandle, HINSTANCE},
    System::{
        LibraryLoader::FreeLibraryAndExitThread,
        Threading::{CreateThread, Sleep, THREAD_CREATION_FLAGS},
    },
};

use crate::main_thread::MainThread;

unsafe extern "system" fn detach_library(module: *mut c_void) -> u32 {
    Sleep(500);
    let module = HINSTANCE(module as isize);
    MainThread::stop().expect("can't stop main thread");
    Sleep(5000);
    println!("second thread");
    FreeLibraryAndExitThread(module, 0);
}

pub unsafe extern "system" fn start_detach_library(module: HINSTANCE) {
    let hndl = CreateThread(
        None,
        0,
        Some(detach_library),
        Some(module.0 as *const c_void),
        THREAD_CREATION_FLAGS(0),
        None,
    )
    .unwrap();
    CloseHandle(hndl);
}
