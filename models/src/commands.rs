use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Command {
    MouseLeftClick { x: i32, y: i32 },
    DetachLibrary,
}
