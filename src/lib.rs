use std::error::Error;
use std::ffi::{OsStr, CStr, c_void};
use std::mem;

use libc::strcat;
use winapi::shared::ntdef::HRESULT;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, LoadLibraryA, GetProcAddress};
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;
use windows::core::{PCSTR};
use windows::Win32::System::Diagnostics::Debug::{ImageNtHeader, IMAGE_DIRECTORY_ENTRY_IMPORT};
use winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL, TRUE, MAX_PATH, FARPROC};
use winapi::um::winnt::{DLL_PROCESS_ATTACH};

type DirectSoundCreateType = unsafe extern "system" fn(_: *const c_void, _: *mut c_void, _: *const c_void) -> HRESULT;

static mut oDirectSoundCreate: Option<DirectSoundCreateType> = None;

#[no_mangle]
pub unsafe extern "C" fn DirectSoundCaptureCreate(_: *const c_void, _: *mut c_void, _: *const c_void) -> HRESULT { -1 }
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCaptureCreate8(_: *const c_void, _: *mut c_void, _: *const c_void) -> HRESULT { -1 }
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCaptureEnumerateA(_: *const c_void, _: LPVOID) -> HRESULT { -1 }
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCaptureEnumerateW(_: *const c_void, _: LPVOID) -> HRESULT { -1 }
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCreate(a: *const c_void, b: *mut c_void, c: *const c_void) -> HRESULT { 
    oDirectSoundCreate.as_ref().unwrap()(a, b, c)
}
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCreate8(a: *const c_void, b: *mut c_void, c: *const c_void) -> HRESULT { 
    oDirectSoundCreate.as_ref().unwrap()(a, b, c)
}
#[no_mangle]
pub unsafe extern "C" fn DirectSoundEnumerateA(_: *const c_void, _: LPVOID) -> HRESULT { -1 }
#[no_mangle]
pub unsafe extern "C" fn DirectSoundEnumerateW(_: *const c_void, _: LPVOID) -> HRESULT { -1 }
#[no_mangle]
pub unsafe extern "C" fn DirectSoundFullDuplexCreate(
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
) -> HRESULT { -1 }
#[no_mangle]
pub unsafe extern "C" fn DllCanUnloadNow() {}
#[no_mangle]
pub unsafe extern "C" fn DllGetClassObject() {}
#[no_mangle]
pub unsafe extern "C" fn GetDeviceID(
    _: *const c_void,
    _: *const c_void,
) -> HRESULT { -1 }


unsafe fn main() -> Result<(), Box<dyn Error>> {
    let mut path = vec![];
    path.resize(MAX_PATH, 0);
    let _ = GetSystemDirectoryA(&mut path[..]);
    let s = String::from_utf8(path)?;
    let s = s + "\\dsound.dll";
    let ss = PCSTR::from_raw(s.as_ptr());
    let lib = LoadLibraryA(ss)?;
    let name = "DirectSoundCreate";
    let hndl = mem::transmute::<unsafe extern "system" fn() -> isize, DirectSoundCreateType>(GetProcAddress(lib, PCSTR::from_raw(name.as_ptr())).unwrap());
    oDirectSoundCreate = Some(hndl);
    
    // rewrite_iat()
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
        main().is_ok() as BOOL
    } else {
        TRUE
    }
}