use serde::{Serialize, Deserialize};
use derivative::Derivative;

#[derive(Serialize, Deserialize)]
pub enum Command {
    MouseLeftClick {
        x: u32,
        y: u32,
    }
}