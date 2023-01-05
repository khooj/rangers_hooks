use ws::Sender;

pub struct ServeHandler {
    sender: Sender,
}

impl ServeHandler {
    pub fn new(sender: Sender) -> Self {
        ServeHandler { sender }
    }
}

impl ws::Handler for ServeHandler {
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
