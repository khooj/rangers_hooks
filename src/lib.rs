use std::error::Error;
use std::ffi::{CString, c_void};
use std::mem;

use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress};
use windows::Win32::System::{SystemInformation::GetSystemDirectoryA, SystemServices::DLL_PROCESS_ATTACH};
use windows::core::{PCSTR, HRESULT};
use windows::Win32::Foundation::{HINSTANCE, MAX_PATH, BOOL};

type LPVOID = *const c_void;
type DWORD = u32;

type DirectSoundEnumerate = unsafe extern "system" fn(_: *mut c_void, _: LPVOID) -> HRESULT;

static mut DIRECT_SOUND_CAPTURE_ENUMERATE_A: Option<DirectSoundEnumerate> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundCaptureEnumerateA(a: *mut c_void, b: LPVOID) -> HRESULT {
    DIRECT_SOUND_CAPTURE_ENUMERATE_A.as_ref().unwrap()(a, b)
}

static mut DIRECT_SOUND_CAPTURE_ENUMERATE_W: Option<DirectSoundEnumerate> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundCaptureEnumerateW(a: *mut c_void, b: LPVOID) -> HRESULT {
    DIRECT_SOUND_CAPTURE_ENUMERATE_W.as_ref().unwrap()(a, b)
}

static mut DIRECT_SOUND_ENUMERATE_A: Option<DirectSoundEnumerate> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundEnumerateA(a: *mut c_void, b: LPVOID) -> HRESULT {
    let gg = DIRECT_SOUND_ENUMERATE_A.as_ref();
    match gg {
        Some(k) => k(a, b),
        None => { println!("empty ptr"); HRESULT(-1) }
    }
}

static mut DIRECT_SOUND_ENUMERATE_W: Option<DirectSoundEnumerate> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundEnumerateW(a: *mut c_void, b: LPVOID) -> HRESULT {
    DIRECT_SOUND_ENUMERATE_W.as_ref().unwrap()(a, b)
}

unsafe fn init_func2(lib: HINSTANCE, method_name: &str, store: &mut Option<DirectSoundEnumerate>) {
    let m = CString::new(method_name).expect("cstring1");
    let func = GetProcAddress(lib, PCSTR::from_raw(m.as_ptr() as *const u8)).expect("can't get proc addr");
    let hndl = mem::transmute::<unsafe extern "system" fn() -> isize, DirectSoundEnumerate>(func);
    store.replace(hndl);
}

unsafe fn main() -> Result<(), Box<dyn Error>> {
    let mut path = vec![];
    path.resize(MAX_PATH as usize, 0);
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
    init_func2(lib, "DirectSoundEnumerateA", &mut DIRECT_SOUND_ENUMERATE_A);
    init_func2(lib, "DirectSoundEnumerateW", &mut DIRECT_SOUND_ENUMERATE_W);
    init_func2(lib, "DirectSoundCaptureEnumerateA", &mut DIRECT_SOUND_CAPTURE_ENUMERATE_A);
    init_func2(lib, "DirectSoundCaptureEnumerateW", &mut DIRECT_SOUND_CAPTURE_ENUMERATE_W);

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
            false.into()
        } else {
            true.into()
        }
    } else {
        true.into()
    }
}