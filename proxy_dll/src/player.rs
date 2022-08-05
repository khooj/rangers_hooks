use widestring::U16CStr;

static mut PLAYER_INFO_PTR: u32 = 0x85b610;

pub fn get_player_struct() -> Option<*const PlayerInfo> {
    unsafe {
        let p = PLAYER_INFO_PTR as *const u32;
        if *p != 0xb1cd15d3 {
            let a = (*p) ^ 0xb1cd15d3;
            Some(a as *const PlayerInfo)
        } else {
            None
        }
    }
}

#[repr(C)]
pub struct PlayerInfo {
    unk_ptr: u32,
    unk_type: u32,
    player_name_ptr: *const u16,
    empty_space0: [u32; 6],
    maybe_current_system_ptr: *const PlanetSystem,
    maybe_previous_system_ptr: *const PlanetSystem,
    empty_space2: [u32; 3],
    some_flag: u32,
    empty_space1: [u32; 220],
    experience: u32,
}

impl PlayerInfo {
    pub unsafe fn name(&self) {
        let s = U16CStr::from_ptr_str(self.player_name_ptr);
        println!("player name: {}", s.display());
    }

    pub fn experience(&self) {
        println!("exp: {}", self.experience);
    }
}

#[repr(C)]
pub struct PlanetSystem {
    empty_space0: [u32; 4],
    name: *const u16,
}
