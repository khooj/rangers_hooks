use std::mem::size_of;

use widestring::U16CStr;

static mut PLAYER_INFO_PTR: u32 = 0x85b610;

pub fn get_player_struct() -> Option<models::SpaceshipInfo> {
    unsafe {
        let p = PLAYER_INFO_PTR as *const u32;
        if *p != 0xb1cd15d3 {
            let a = (*p) ^ 0xb1cd15d3;
            let a = &*(a as *const SpaceshipInfo);
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
struct SpaceshipInfo {
    unk_ptr: u32, // 0
    unk_type: u32,
    player_name_ptr: *const u16,
    empty_space0: [u32; 2],
    x: f32,
    y: f32,
    unk4: [u32; 2],
    maybe_current_system_ptr: *const PlanetSystem,
    maybe_previous_system_ptr: *const PlanetSystem,
    maybe_some_ship_ptr: u32,
    empty_space2: [u32; 2],
    some_flag: u32, // 14 u32
    unk5: [u32; 34],
    speed: u32,
    empty_space4: [u32; 10],
    money: u32,
    unk1: u32,
    hull_ptr: *const Unk_HullData, // 62
    empty_space1: [u32; 172],
    experience: u32, // 235
    unk1_m: [u32; 2],
    unk_ptr1: *const (), // used at 0x75f214
    unk_ptr2: *const (),
    unk_ptr3: *const (),
    unk6: [u32; 12],
    maybe_last_attacked_me_ship: *const SpaceshipInfo,
    maybe_last_friended_ship: *const SpaceshipInfo,
    unk7: [u32; 14],
    x_movement: f32,
    y_movement: f32,
    empty_space3: [u32; 5],
    ship_info: *const ShipInfo,
    ship_type: *const u16,
    unk8: [u32; 9],
    unk_x: f32,
    unk_y: f32,
}

impl SpaceshipInfo {
    fn name(&self) -> String {
        unsafe {
            let s = U16CStr::from_ptr_str(self.player_name_ptr);
            s.to_string_lossy()
        }
    }

    fn clone_as_model(&self) -> models::SpaceshipInfo {
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
            let last_attacked_me = if (self.maybe_last_attacked_me_ship as u32) != 0 {
                Some(Box::new((*self.maybe_last_attacked_me_ship).clone_as_model()))
            } else {
                None
            };
            let last_friended = if (self.maybe_last_friended_ship as u32) != 0 {
                Some(Box::new((*self.maybe_last_friended_ship).clone_as_model()))
            } else {
                None
            };
            models::SpaceshipInfo {
                experience: self.experience,
                player_name: self.name(),
                current_system: (*self.maybe_current_system_ptr).clone_as_model(),
                previous_system,
                hull,
                money: self.money,
                x: self.x,
                y: self.y,
                speed: self.speed,
                last_attacked_me,
                last_friended,
                x_movement: self.x_movement,
                y_movement: self.y_movement,
                x_unk: self.unk_x,
                y_unk: self.unk_y,
            }
        }
    }
}

#[repr(C)]
struct PlanetSystem {
    empty_space0: [u32; 4],
    name: *const u16,
    unk1: [u32; 4],
    maybe_planets: *const Unk_SystemObject,
    asteroids: *const Unk_SystemObject,
    spaceships: *const Unk_SystemObject,
    unk_objects2: u32,
    unk_objects3: u32,
    unk_objects4: u32,
}

impl PlanetSystem {
    fn name(&self) -> String {
        unsafe {
            let s = U16CStr::from_ptr_str(self.name);
            s.to_string_lossy()
        }
    }

    fn planets(&self) -> Vec<models::Planet> {
        unsafe {
            (*self.maybe_planets)
                .objects
                .iter()
                .map(|e| {
                    let k = (*e).planet_ptr;
                    let name = U16CStr::from_ptr_str((*k).name);
                    models::Planet {
                        name: name.to_string_lossy(),
                    }
                })
                .collect()
        }
    }

    fn spaceships(&self) -> Vec<models::SpaceshipInfo> {
        unsafe {
            (*self.spaceships)
                .objects
                .iter()
                .map(|e| {
                    let k = (*e).spaceship_ptr;
                    (*k).clone_as_model()
                })
                .collect()
        }
    }

    fn clone_as_model(&self) -> models::PlanetSystem {
        models::PlanetSystem {
            name: self.name(),
            planets: self.planets(),
            // probably app crashes here because of overflow
            // in this vec we have player ship too and it can
            // cause infinite cycle
            // spaceships: self.spaceships(),
        }
    }
}

#[repr(C)]
struct Unk_SystemObject {
    maybe_const_marker: u32,
    objects: SystemObjectsRange,
    unk1: [u32; 3],
}

#[repr(C)]
struct SystemObjectsRange {
    objects_ptr: *const Unk_SystemObjectPtr,
    count: u32,
}

impl SystemObjectsRange {
    pub fn iter(&self) -> SystemObjectsRangeIter {
        SystemObjectsRangeIter {
            count: self.count,
            current: 0,
            objects_ptr: self.objects_ptr,
        }
    }
}

struct SystemObjectsRangeIter {
    count: u32,
    current: u32,
    objects_ptr: *const Unk_SystemObjectPtr,
}

impl Iterator for SystemObjectsRangeIter {
    type Item = *const Unk_SystemObjectPtr;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.count {
            return None;
        }

        let m = self.objects_ptr as u32;
        let m = m + self.current * size_of::<Unk_SystemObjectPtr>() as u32;
        let m = m as *const Unk_SystemObjectPtr;
        self.current += 1;
        Some(m)
    }
}

#[repr(C)]
union Unk_SystemObjectPtr {
    planet_ptr: *const PlanetInfo,
    asteroid_ptr: *const AsteroidInfo,
    spaceship_ptr: *const SpaceshipInfo,
}

#[repr(C)]
struct PlanetInfo {
    unk1: [u32; 5],
    name: *const u16,
    system_ptr: *const PlanetSystem,
}

#[repr(C)]
struct AsteroidInfo {
    unk1: [u32; 2],
    system_ptr: *const PlanetSystem,
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
    player_info_ptr: *const SpaceshipInfo,
}

impl Unk_HullData {
    fn clone_as_model(&self) -> models::HullData {
        models::HullData { hp: self.hull_hp }
    }
}

// looks like linked list with ships types and asteroids (?)
#[repr(C)]
struct ShipInfo {
    unk1: u32,
    
}