use std::error::Error;
use std::ffi::{OsStr, CStr, CString, c_void};
use std::mem;
use std::path::PathBuf;
use std::str::FromStr;

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

type DirectSoundCreateType = unsafe extern "system" fn(_: *const c_void, _: *mut LPVOID, _: *const c_void) -> HRESULT;

static mut direct_sound_capture_create: Option<DirectSoundCreateType> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundCaptureCreate(a: *const c_void, b: *mut LPVOID, c: *const c_void) -> HRESULT {
    direct_sound_capture_create.as_ref().unwrap()(a, b, c)
}

static mut direct_sound_capture_create8: Option<DirectSoundCreateType> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundCaptureCreate8(a: *const c_void, b: *mut LPVOID, c: *const c_void) -> HRESULT {
    direct_sound_capture_create8.as_ref().unwrap()(a, b, c)
}

static mut direct_sound_create: Option<DirectSoundCreateType> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundCreate(a: *const c_void, b: *mut LPVOID, c: *const c_void) -> HRESULT { 
    direct_sound_create.as_ref().unwrap()(a, b, c)
}

static mut direct_sound_create8: Option<DirectSoundCreateType> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundCreate8(a: *const c_void, b: *mut LPVOID, c: *const c_void) -> HRESULT { 
    direct_sound_create8.as_ref().unwrap()(a, b, c)
}

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

type DirectSoundFullDuplexCreateType = unsafe extern "system" fn(
    _: *const c_void,
    _: *const c_void,
    _: *const c_void,
    _: *const c_void,
    _: *const c_void,
    _: *const c_void,
    _: *const c_void,
    _: *const c_void,
    _: *const c_void,
    _: *const c_void,
) -> HRESULT;

static mut direct_sound_full_duplex_create: Option<DirectSoundFullDuplexCreateType> = None;
#[no_mangle]
pub unsafe extern "system" fn DirectSoundFullDuplexCreate(
    a: *const c_void,
    b: *const c_void,
    c: *const c_void,
    d: *const c_void,
    e: *const c_void,
    f: *const c_void,
    g: *const c_void,
    h: *const c_void,
    i: *const c_void,
    j: *const c_void,
) -> HRESULT {
    direct_sound_full_duplex_create.as_ref().unwrap()(a, b, c, d, e, f, g, h, i, j)
}

type GetDeviceIDType = unsafe extern "system" fn(_: *const c_void, _: *const c_void) -> HRESULT;

static mut get_device_id: Option<GetDeviceIDType> = None;
#[no_mangle]
pub unsafe extern "system" fn GetDeviceID(
    a: *const c_void,
    b: *const c_void,
) -> HRESULT { get_device_id.as_ref().unwrap()(a, b) }

unsafe fn init_func1(lib: HINSTANCE, method_name: &str, store: &mut Option<DirectSoundCreateType>) {
    let func = GetProcAddress(lib, PCSTR::from_raw(method_name.as_ptr())).expect("can't get proc addr");
    let hndl = mem::transmute::<unsafe extern "system" fn() -> isize, DirectSoundCreateType>(func);
    store.replace(hndl);
}

unsafe fn init_func2(lib: HINSTANCE, method_name: &str, store: &mut Option<DirectSoundEnumerate>) {
    let m = CString::new(method_name).expect("cstring1");
    let func = GetProcAddress(lib, PCSTR::from_raw(m.as_ptr() as *const u8)).expect("can't get proc addr");
    let hndl = mem::transmute::<unsafe extern "system" fn() -> isize, DirectSoundEnumerate>(func);
    store.replace(hndl);
}

unsafe fn init_func3(lib: HINSTANCE, method_name: &str, store: &mut Option<DirectSoundFullDuplexCreateType>) {
    let func = GetProcAddress(lib, PCSTR::from_raw(method_name.as_ptr())).expect("can't get proc addr");
    let hndl = mem::transmute::<unsafe extern "system" fn() -> isize, DirectSoundFullDuplexCreateType>(func);
    store.replace(hndl);
}

unsafe fn init_func4(lib: HINSTANCE, method_name: &str, store: &mut Option<GetDeviceIDType>) {
    let func = GetProcAddress(lib, PCSTR::from_raw(method_name.as_ptr())).expect("can't get proc addr");
    let hndl = mem::transmute::<unsafe extern "system" fn() -> isize, GetDeviceIDType>(func);
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
    // init_func1(lib, "DirectSoundCreate", &mut direct_sound_create);
    // init_func1(lib, "DirectSoundCreate8", &mut direct_sound_create8);
    // init_func1(lib, "DirectSoundCaptureCreate", &mut direct_sound_capture_create);
    // init_func1(lib, "DirectSoundCaptureCreate8", &mut direct_sound_capture_create8);
    init_func2(lib, "DirectSoundEnumerateA", &mut direct_sound_enumerate_a);
    init_func2(lib, "DirectSoundEnumerateW", &mut direct_sound_enumerate_w);
    init_func2(lib, "DirectSoundCaptureEnumerateA", &mut direct_sound_capture_enumerate_a);
    init_func2(lib, "DirectSoundCaptureEnumerateW", &mut direct_sound_capture_enumerate_w);
    // init_func3(lib, "DirectSoundFullDuplexCreate", &mut direct_sound_full_duplex_create);
    // init_func4(lib, "GetDeviceID", &mut get_device_id);

    Ok(())
}

// unsafe fn rewrite_iat() -> Result<(), Box<dyn Error>> {
//     use std::ptr::{null, null_mut};
//     use std::os::windows::ffi::OsStrExt;

//     let image_base_inst = GetModuleHandleA(PCSTR::null())?;

//     let image_base = image_base_inst.0 as *const c_void;
//     // let dos_headers = image_base as PIMAGE_DOS_HEADER;
//     // let nt_headers = (image_base + (*dos_headers).e_lfanew as usize) as PIMAGE_NT_HEADERS;
//     let nt_headers = ImageNtHeader(image_base);

//     // let imports_directory = (*nt_headers).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT as usize] as IMAGE_DATA_DIRECTORY;
//     let imports_directory = (*nt_headers).OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize];
//     Descri
//     let mut import_descriptor = (image_base + imports_directory.VirtualAddress as usize) as PIMAGE_IMPORT_DESCRIPTOR;

//     let mut library: HMODULE = NULL as HMODULE;

//     while (*import_descriptor).Name != 0 {
//         let library_name = ((*import_descriptor).Name as usize + image_base) as LPCSTR;
//         let name = CStr::from_ptr(library_name);
//         if name.to_str().unwrap() == "dsound.dll" {
//             library = LoadLibraryA(library_name);
//             break;
//         }

//         import_descriptor = (import_descriptor as DWORD_PTR + 1usize) as PIMAGE_IMPORT_DESCRIPTOR;
//     }

//     if library.is_null() {
//         panic!("library not loaded");
//     }

//     let mut original_first_thunk = (image_base + *(*import_descriptor).u.OriginalFirstThunk() as usize) as PIMAGE_THUNK_DATA;
//     let mut first_thunk = (image_base + (*import_descriptor).FirstThunk as usize) as PIMAGE_THUNK_DATA;

//     while *(*original_first_thunk).u1.AddressOfData() != 0 {
//         let function_name = (image_base as DWORD_PTR + *(*original_first_thunk).u1.AddressOfData() as usize) as PIMAGE_IMPORT_BY_NAME;
//         original_first_thunk = (original_first_thunk as DWORD_PTR + 1usize) as PIMAGE_THUNK_DATA;
//         first_thunk = (first_thunk as DWORD_PTR + 1usize) as PIMAGE_THUNK_DATA;
//     }

//     Ok(())
// }

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