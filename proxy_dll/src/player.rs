use widestring::U16CStr;

static mut PLAYER_INFO_PTR: u32 = 0x85b610;

pub fn get_player_struct() -> Option<models::PlayerInfo> {
    unsafe {
        let p = PLAYER_INFO_PTR as *const u32;
        if *p != 0xb1cd15d3 {
            let a = (*p) ^ 0xb1cd15d3;
            let a = &*(a as *const PlayerInfo);
            // we still have some penalty for struct creation but for now its ok
            // println!("player info raw: {:?}", a);
            Some(a.clone_as_model())
        } else {
            None
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct PlayerInfo {
    unk_ptr: u32,
    unk_type: u32,
    player_name_ptr: *const u16,
    empty_space0: [u32; 6],
    maybe_current_system_ptr: *const PlanetSystem,
    maybe_previous_system_ptr: *const PlanetSystem,
    empty_space2: [u32; 3],
    some_flag: u32, // 15 u32
    empty_space4: [u32; 45],
    money: u32,
    unk1: u32,
    hull_ptr: *const Unk_HullData,
    empty_space1: [u32; 172], // 235 u32
    experience: u32,
    unk1_m: [u32; 2],
    unk_ptr1: *const Unknown1, // used at 0x75f214
    empty_space3: [u32; 37],
    ship_type: *const u16,
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
            let previous_system = if (self.maybe_previous_system_ptr as u32) == 0 {
                None
            } else {
                Some((*self.maybe_previous_system_ptr).clone_as_model())
            };
            let hull = if (self.hull_ptr as u32) == 0 {
                None
            } else {
                Some((*self.hull_ptr).clone_as_model())
            };
            models::PlayerInfo {
                experience: self.experience,
                player_name: self.name(),
                current_system: (*self.maybe_current_system_ptr).clone_as_model(),
                previous_system,
                hull,
                money: self.money,
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

#[repr(C)]
struct Unknown1 {
    unk1: [u32; 2],
    some_ptr: *const (),
}

#[repr(C)]
struct Unk_HullData {
    unk1: [u32; 24],
    hull_hp: u32,
    maybe_unk_constant: u32,
    unk_ffff: u32,
    player_info_ptr: *const PlayerInfo,
}

impl Unk_HullData {
    fn clone_as_model(&self) -> models::HullData {
        models::HullData {
            hp: self.hull_hp,
        }
    }
}
