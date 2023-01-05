pub mod commands;

use serde::{Serialize, Deserialize};
use derivative::Derivative;

#[derive(Serialize, Deserialize)]
pub enum EncodedMessage {
    NoMessage,
    PlayerInfo(SpaceshipInfo),
    EmptyPlayerInfo,
}

#[derive(Serialize, Deserialize, Debug, Derivative, Clone, Default)]
#[derivative(PartialEq)]
pub struct SpaceshipInfo {
    pub core: SpaceshipInfoCore,
    pub current_system: Option<PlanetSystem>,
    // can be empty at game start
    pub previous_system: Option<PlanetSystem>,
    pub last_attacked_me: Option<SpaceshipInfoCore>,
    pub last_friended: Option<SpaceshipInfoCore>,
}

#[derive(Serialize, Deserialize, Debug, Derivative, Clone, Default)]
#[derivative(PartialEq)]
pub struct SpaceshipInfoCore {
    // #[derivative(PartialEq="ignore")]
    pub player_name: String,
    pub experience: u32,
    pub money: u32,
    pub hull: Option<HullData>,
    pub speed: u32,
    pub x: f32,
    pub y: f32,
    pub x_movement: f32,
    pub y_movement: f32,
}


#[derive(Serialize, Deserialize, Debug, Derivative, Clone)]
#[derivative(PartialEq)]
pub struct PlanetSystem {
    pub name: String,
    pub planets: Vec<Planet>,
    pub spaceships: Vec<SpaceshipInfoCore>,
}

#[derive(Serialize, Deserialize, Debug, Derivative, Clone)]
#[derivative(PartialEq)]
pub struct Planet {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Derivative, Clone)]
#[derivative(PartialEq)]
pub struct HullData {
    pub hp: u32,
}
