use std::fmt::Display;
use std::mem::size_of;
// code and idea taken from https://github.com/eur0pa/ds2fix64
use std::os::raw::c_void;

use windows::Win32::System::Diagnostics::Debug::ImageNtHeader;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::{VirtualQuery, MEMORY_BASIC_INFORMATION};
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE};
use windows::core::PCSTR;

#[derive(Debug)]
pub enum SigscanError {
    CantFindSignature,
    ImageInfo,
    VirtualQuery,
    FindBase,
    WinError(windows::core::Error),
}

impl From<windows::core::Error> for SigscanError {
    fn from(v: windows::core::Error) -> Self {
        SigscanError::WinError(v)
    }
}

impl Display for SigscanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason = match *self {
            Self::CantFindSignature => "cant find signature",
            Self::ImageInfo => "cant call image info",
            Self::VirtualQuery => "cant call virtual query",
            Self::FindBase => "cant find base",
            Self::WinError(_) => "win call error",
        };
        write!(f, "{}", reason)
    }
}

pub struct Signature {
    pub signature: &'static [u8],
    pub mask: &'static [u8],
    pub ret: u32,
}

pub unsafe fn find_signature(base_addr: *const c_void, image_len: u32, sig: &Signature) -> Result<u32, SigscanError> {
    let base_addr = base_addr as u32;
    let mut scan = base_addr;
    let mut max_len = 0;

    while scan < base_addr + image_len - sig.signature.len() as u32 {
        let mut sz_len = 0;

        for i in 0..sig.signature.len() {
            let is_sig_byte = *((scan+i as u32) as *const u8) == sig.signature[i];
            let is_mask_byte = sig.mask[i] == b'?';
            if !(is_sig_byte || is_mask_byte) {
                break;
            }

            sz_len += 1;
        }

        if sz_len > max_len {
            max_len = sz_len;
        }

        if sz_len == sig.signature.len() {
            return Ok(scan);
        }

        scan = scan + 1;
    }

    println!("stopped at addr: {} with max_len: {}", scan, max_len);
    return Err(SigscanError::CantFindSignature);
}

pub unsafe fn get_image_info() -> Result<(*const c_void, u32), SigscanError> {
    let module = GetModuleHandleA(PCSTR::null())?;
    if module.0 == 0 {
        return Err(SigscanError::ImageInfo);
    }

    let mut mem_info: MEMORY_BASIC_INFORMATION = MEMORY_BASIC_INFORMATION::default();

    let res = VirtualQuery(module.0 as *const c_void, &mut mem_info, size_of::<MEMORY_BASIC_INFORMATION>());
    if res == 0 {
        return Err(SigscanError::VirtualQuery);
    }

    let dos_headers: *mut IMAGE_DOS_HEADER = module.0 as *mut IMAGE_DOS_HEADER;
    let nt_headers = ImageNtHeader(module.0 as *const c_void);
    let is_dos_signature = (*dos_headers).e_magic == IMAGE_DOS_SIGNATURE;
    let is_nt_signature = (*nt_headers).Signature == IMAGE_NT_SIGNATURE;
    if is_dos_signature && is_nt_signature {
        let base = mem_info.AllocationBase as *const c_void;
        let len = (*nt_headers).OptionalHeader.SizeOfImage;
        return Ok((base, len))
    }

    return Err(SigscanError::FindBase);
}