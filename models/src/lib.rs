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
    #[derivative(PartialEq="ignore")]
    pub player_name: String,
    pub experience: u32,
    pub money: u32,
    pub current_system: Option<PlanetSystem>,
    // can be empty at game start
    pub previous_system: Option<PlanetSystem>,
    pub hull: Option<HullData>,
    pub speed: u32,
    pub x: f32,
    pub y: f32,
    // boxed values should be transferred somehow
    // if serializer does not handle it already
    pub last_attacked_me: Option<Box<SpaceshipInfo>>,
    pub last_friended: Option<Box<SpaceshipInfo>>,
    pub x_movement: f32,
    pub y_movement: f32,
}

#[derive(Serialize, Deserialize, Debug, Derivative, Clone)]
#[derivative(PartialEq)]
pub struct PlanetSystem {
    pub name: String,
    pub planets: Vec<Planet>,
    pub spaceships: Vec<SpaceshipInfo>,
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
