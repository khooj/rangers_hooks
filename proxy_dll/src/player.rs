use widestring::U16CStr;

static mut PLAYER_INFO_PTR: u32 = 0x85b610;

pub fn get_player_struct() -> Option<models::PlayerInfo> {
    unsafe {
        let p = PLAYER_INFO_PTR as *const u32;
        if *p != 0xb1cd15d3 {
            let a = (*p) ^ 0xb1cd15d3;
            let a = &*(a as *const PlayerInfo);
            // we still have some penalty for struct creation but for now its ok
            Some(a.clone_as_model())
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
    fn name(&self) -> String {
        unsafe {
            let s = U16CStr::from_ptr_str(self.player_name_ptr);
            s.to_string_lossy()
        }
    }

    fn clone_as_model(&self) -> models::PlayerInfo {
        unsafe {
            models::PlayerInfo {
                experience: self.experience,
                player_name: self.name(),
                current_system: (*self.maybe_current_system_ptr).clone_as_model(),
                previous_system: (*self.maybe_previous_system_ptr).clone_as_model(),
            }
        }
    }
}

#[repr(C)]
pub struct PlanetSystem {
    empty_space0: [u32; 4],
    name: *const u16,
}

impl PlanetSystem {
    fn name(&self) -> String {
        unsafe {
            let s = U16CStr::from_ptr_str(self.name);
            s.to_string_lossy()
        }
    }
    fn clone_as_model(&self) -> models::PlanetSystem {
        models::PlanetSystem { name: self.name() }
    }
}
