use std::error::Error;
use std::ffi::{OsStr, CStr, CString, c_void};
use std::mem;

use libc::strcat;
use winapi::shared::ntdef::{HRESULT };
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, LoadLibraryA, GetProcAddress};
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;
use windows::core::{PCSTR};
use windows::Win32::System::Diagnostics::Debug::{ImageNtHeader, IMAGE_DIRECTORY_ENTRY_IMPORT};
use windows::Win32::Foundation::HINSTANCE;
use winapi::shared::minwindef::{DWORD, LPVOID, BOOL, TRUE, FALSE, MAX_PATH, FARPROC};
use winapi::um::winnt::{DLL_PROCESS_ATTACH};

type DirectSoundEnumerate = unsafe extern "system" fn(_: *mut c_void, _: LPVOID) -> HRESULT;

static mut direct_sound_capture_enumerate_a: Option<DirectSoundEnumerate> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundCaptureEnumerateA(a: *mut c_void, b: LPVOID) -> HRESULT {
    direct_sound_capture_enumerate_a.as_ref().unwrap()(a, b)
}

static mut direct_sound_capture_enumerate_w: Option<DirectSoundEnumerate> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundCaptureEnumerateW(a: *mut c_void, b: LPVOID) -> HRESULT {
    direct_sound_capture_enumerate_w.as_ref().unwrap()(a, b)
}

static mut direct_sound_enumerate_a: Option<DirectSoundEnumerate> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundEnumerateA(a: *mut c_void, b: LPVOID) -> HRESULT {
    let gg = direct_sound_enumerate_a.as_ref();
    match gg {
        Some(k) => k(a, b),
        None => { println!("empty ptr"); -1 }
    }
}

static mut direct_sound_enumerate_w: Option<DirectSoundEnumerate> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundEnumerateW(a: *mut c_void, b: LPVOID) -> HRESULT {
    direct_sound_enumerate_w.as_ref().unwrap()(a, b)
}

unsafe fn init_func2(lib: HINSTANCE, method_name: &str, store: &mut Option<DirectSoundEnumerate>) {
    let m = CString::new(method_name).expect("cstring1");
    let func = GetProcAddress(lib, PCSTR::from_raw(m.as_ptr() as *const u8)).expect("can't get proc addr");
    let hndl = mem::transmute::<unsafe extern "system" fn() -> isize, DirectSoundEnumerate>(func);
    store.replace(hndl);
}

unsafe fn main() -> Result<(), Box<dyn Error>> {
    let mut path = vec![];
    path.resize(MAX_PATH, 0);
    let _ = GetSystemDirectoryA(&mut path[..]);

    let s = String::from_iter(
        path
        .into_iter()
        .filter_map(|c| if c != 0 { char::from_u32(c as u32) } else { None })
        .chain("\\dsound.dll\0".chars())
    );
    let ss = PCSTR::from_raw(s.as_ptr());
    println!("path: {}", ss.to_string().expect("to_string"));
    let lib = LoadLibraryA(ss)?;
    init_func2(lib, "DirectSoundEnumerateA", &mut direct_sound_enumerate_a);
    init_func2(lib, "DirectSoundEnumerateW", &mut direct_sound_enumerate_w);
    init_func2(lib, "DirectSoundCaptureEnumerateA", &mut direct_sound_capture_enumerate_a);
    init_func2(lib, "DirectSoundCaptureEnumerateW", &mut direct_sound_capture_enumerate_w);

    Ok(())
}

#[no_mangle]
pub unsafe extern "system" fn DllMain(
    _module: HINSTANCE,
    call_reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    if call_reason == DLL_PROCESS_ATTACH {
        AllocConsole();
        let r = main();
        if let Err(v) = r {
            println!("error: {}", v);
            FALSE
        } else {
            TRUE
        }
    } else {
        TRUE
    }
}