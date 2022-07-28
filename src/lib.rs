mod sigscan;
use sigscan::*;

use std::error::Error;
use std::ffi::{CString, c_void};
use std::mem;
use std::arch::asm;

use detour::{RawDetour};
use windows::Win32::System::Console::AllocConsole;
use windows::Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress, DisableThreadLibraryCalls};
use windows::Win32::System::{SystemInformation::GetSystemDirectoryA, SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH}};
use windows::core::{PCSTR, HRESULT};
use windows::Win32::Foundation::{HINSTANCE, MAX_PATH, BOOL};
use widestring::{U16CStr};

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

const SAVE_PLAYER_SIGNATURE: Signature = Signature{
    signature: &[
        0x55,
        0x8B, 0xEC,
        0x83, 0xC4, 0xF8,
        0x89, 0x55, 0xF8,
        0x89, 0x45, 0xFC,
        0x83, 0x7D, 0xF8, 0x00,
        0x74, 0x48,
        0x8B, 0x45, 0xFC,
        0x35, 0xD3, 0x15, 0xCD, 0xB1,
        0xA3, 0x00, 0x00, 0x00, 0x00,
        0x83, 0x7D, 0xFC, 0x00,
        0x75, 0x0C,
        0x8B, 0x45, 0xF8,
        0xC7, 0x40, 0x28, 0xFF, 0xFF, 0xFF, 0xFF,
        0xEB, 0x29,
        0x8B, 0x45, 0xF8,
        0x83, 0x78, 0x3C, 0x00,
        0x75, 0x0C,
        0x8B, 0x45, 0xF8,
        0xC7, 0x40, 0x28, 0xFF, 0xFF, 0xFF, 0xFF,
        0xEB, 0x14,
        0x8B, 0x55, 0xFC,
        0x8B, 0x45, 0xF8,
        0x8B, 0x40, 0x3C,
        0xE8, 0x40, 0xC1, 0xE9, 0xFF,
        0x8B, 0x55, 0xF8,
        0x89, 0x42, 0x28,
        0x59,
        0x59,
        0x5D,
        0xC3,
    ],
    mask: &[
        b'x', 
        b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', b'x', 
        b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', b'x', b'x', 
        b'x', b'?', b'?', b'?', b'?',
        b'x', b'x', b'x', b'x', 
        b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', b'x', b'x', b'x', b'x', 
        b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', b'x', 
        b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', b'x', b'x', b'x', b'x', 
        b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', b'x', b'x', 
        b'x', 
        b'x', 
        b'x', 
        b'x', 
    ],
    ret: 0,
};

#[repr(C)]
struct PlayerInfo {
    unk_ptr: u32,
    unk_type: u32,
    player_name_ptr: *const u16,
}

static mut SAVE_PLAYER_HOOK: Option<RawDetour> = None;


unsafe fn save_player() {
    let a: u32;
    let b: u32;

    asm!(
        "", 
        out("eax") a,
        out("edx") b,
    );

    let href = SAVE_PLAYER_HOOK.as_ref().expect("empty player hook");
    let orig: fn() = mem::transmute(href.trampoline());
    println!("a is {}\nb is {}", a, b);

    let player_info = a as *const PlayerInfo;

    let s = U16CStr::from_ptr_str((*player_info).player_name_ptr);
    println!("player name: {}", s.display());

    asm!(
        "",
        in("eax") a,
        in("edx") b,
    );

    orig();
}

unsafe fn hook_save_player(addr: u32) {
    let hook = RawDetour::new(addr as *const (), save_player as *const ()).expect("cant get raw detour");
    hook.enable().expect("can't enable hook");
    SAVE_PLAYER_HOOK = Some(hook);
}

unsafe fn remove_hook() {
    let hook = SAVE_PLAYER_HOOK.take();
    let hook = match hook {
        Some(k) => k,
        None => return,
    };
    hook.disable().expect("cant disable hook");
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
    let lib = LoadLibraryA(ss)?;

    init_func2(lib, "DirectSoundEnumerateA", &mut DIRECT_SOUND_ENUMERATE_A);
    init_func2(lib, "DirectSoundEnumerateW", &mut DIRECT_SOUND_ENUMERATE_W);
    init_func2(lib, "DirectSoundCaptureEnumerateA", &mut DIRECT_SOUND_CAPTURE_ENUMERATE_A);
    init_func2(lib, "DirectSoundCaptureEnumerateW", &mut DIRECT_SOUND_CAPTURE_ENUMERATE_W);

    let (base, image_len) = get_image_info().expect("cant get image info");
    println!("base addr: {}, image_len: {}", base as u32, image_len);
    let sig_start = find_signature(base, image_len, &SAVE_PLAYER_SIGNATURE).expect("can't get signature");
    println!("found sig addr: {}", sig_start as u32);

    hook_save_player(sig_start);

    Ok(())
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
        remove_hook();
        true.into()
    } else {
        true.into()
    }
}