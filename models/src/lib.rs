use serde::{Serialize, Deserialize};
use derivative::Derivative;

#[derive(Serialize, Deserialize)]
pub enum EncodedMessage {
    NoMessage,
    PlayerInfo(PlayerInfo),
    EmptyPlayerInfo,
}

#[derive(Serialize, Deserialize, Debug, Derivative, Clone)]
#[derivative(PartialEq)]
pub struct PlayerInfo {
    #[derivative(PartialEq="ignore")]
    pub player_name: String,
    pub experience: u32,
    pub current_system: PlanetSystem,
    pub previous_system: PlanetSystem,
}

#[derive(Serialize, Deserialize, Debug, Derivative, Clone)]
#[derivative(PartialEq)]
pub struct PlanetSystem {
    pub name: String,
}
