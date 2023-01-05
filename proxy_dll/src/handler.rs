use models::commands::Command;
use spmc::{Receiver, TryRecvError};
use std::sync::atomic::{AtomicU8, Ordering};
use windows::Win32::Foundation::HINSTANCE;
use ws::{util::Token, Message, Sender};

const CHECK_EVENT: Token = Token(666);

pub static HANDLERS_COUNT: AtomicU8 = AtomicU8::new(0);

pub struct Handler {
    sender: Sender,
    sub: Receiver<Vec<u8>>,
    instance: HINSTANCE,
}

impl Handler {
    pub fn new(sender: Sender, sub: Receiver<Vec<u8>>, module: HINSTANCE) -> Self {
        Handler {
            sender,
            sub,
            instance: module,
        }
    }
}

impl ws::Handler for Handler {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        HANDLERS_COUNT.fetch_add(1, Ordering::SeqCst);
        self.sender.timeout(90, CHECK_EVENT)
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        HANDLERS_COUNT.fetch_sub(1, Ordering::SeqCst);
        println!("closing");
    }

    fn on_timeout(&mut self, event: Token) -> ws::Result<()> {
        if event == CHECK_EVENT {
            let msg = self.sub.try_recv();
            if msg.is_err() {
                if let Err(TryRecvError::Disconnected) = msg {
                    eprintln!("error getting message for send: {}", msg.err().unwrap());
                }
                return self.sender.timeout(100, CHECK_EVENT);
            }
            let m = msg.unwrap();
            if let Err(e) = self.sender.send(Message::Binary(m)) {
                eprintln!("error on sending ws, shutting down conn: {}", e);
                return self.sender.close(ws::CloseCode::Error);
            }
        }
        self.sender.timeout(100, CHECK_EVENT)
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        if let ws::Message::Binary(data) = msg {
            let data: Command = bincode::deserialize(&data).expect("can't deserialize");

            match data {
                Command::MouseLeftClick { x, y } => {
                    println!("mouse left click command: {} {}", x, y);
                    unsafe {
                        super::commands::mouse_left_click(self.instance, x, y);
                    }
                }
            }
        }
        Ok(())
    }
}
