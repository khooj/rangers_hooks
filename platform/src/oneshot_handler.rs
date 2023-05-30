use models::commands::Command;
use ws::{Handler, Sender};

pub struct OneshotHandler {
    sender: Sender,
    cmd: Command,
}

impl Handler for OneshotHandler {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        let v = bincode::serialize(&self.cmd).expect("cant serialize");
        self.sender.send(ws::Message::Binary(v))?;
        self.sender.close(ws::CloseCode::Normal)
    }
}

pub struct OneshotHandlerFactory {}

impl OneshotHandlerFactory {
    pub fn mouse_down(x: i32, y: i32) -> impl Fn(Sender) -> OneshotHandler {
        move |s| OneshotHandler {
            sender: s,
            cmd: Command::MouseLeftClick { x, y },
        }
    }

    pub fn unload() -> impl Fn(Sender) -> OneshotHandler {
        move |s| OneshotHandler {
            sender: s,
            cmd: Command::DetachLibrary,
        }
    }
}
