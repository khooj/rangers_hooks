use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum EncodedMessage {
    PlayerInfo(PlayerInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInfo {
    pub player_name: String,
    pub experience: u32,
}
