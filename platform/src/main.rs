use ctrlc;
use ws::{connect, Builder, Sender, Settings};

struct Handler {
    sender: Sender,
}

impl Handler {
    pub fn new(sender: Sender) -> Self {
        Handler { sender }
    }
}

impl ws::Handler for Handler {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        match msg {
            ws::Message::Binary(v) => {
                let msg: models::EncodedMessage =
                    bincode::deserialize(&v[..]).expect("can't deserialize method");
                match msg {
                    models::EncodedMessage::PlayerInfo(m) => println!("got player info: {:?}", m),
                    _ => println!("unhandled msg"),
                };
            }
            _ => {
                println!("ignored text msg");
            }
        };
        Ok(())
    }
}

fn main() {
    connect("ws://localhost:3012", Handler::new).expect("can't connect");
}
