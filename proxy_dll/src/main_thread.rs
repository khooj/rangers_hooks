use crate::handler::{Handler, HANDLERS_COUNT};

use super::player::*;
use models::EncodedMessage;
use std::{
    thread::{self, JoinHandle},
    time::Duration, sync::atomic::Ordering,
};
use thiserror::Error;
use ws::{Builder, Settings};

#[derive(Error, Debug)]
pub enum MainThreadError {}

pub struct MainThread {}

static mut MAIN_THREAD: Option<JoinHandle<()>> = None;
static mut DATA_THREAD: Option<JoinHandle<()>> = None;

impl MainThread {
    pub fn start() -> Result<(), MainThreadError> {
        let (mut tx, rx) = spmc::channel();

        let hndl = Some(thread::spawn(move || {
            let mut s = Settings::default();
            s.max_connections = 1;
            s.panic_on_capacity = true;
            let w = Builder::new()
                .with_settings(s)
                .build(|out| Handler::new(out, rx.clone()))
                .expect("can't create ws");
            w.listen("127.0.0.1:3012").expect("can't start ws");
        }));
        let hndl2 = Some(thread::spawn(move || loop {
            let c = HANDLERS_COUNT.load(Ordering::SeqCst);
            println!("h count: {}", c);
            if c == 0 {
                println!("no handler?");
                std::thread::sleep(Duration::from_millis(250));
                continue;
            }

            if let Some(p) = get_player_struct() {
                let m = p.clone_as_model();
                let m = EncodedMessage::PlayerInfo(m);
                let m = bincode::serialize(&m).expect("can't serialize message");
                // ignore SendError if no receivers exist, we are ok
                let _ = tx.send(m);
            }

            std::thread::sleep(Duration::from_millis(250));
        }));
        unsafe {
            MAIN_THREAD = hndl;
            DATA_THREAD = hndl2;
        }
        Ok(())
    }

    // dont sure if i need to manually stop thread
    pub fn stop() -> Result<(), MainThreadError> {
        unsafe {
            let _ = DATA_THREAD
                .take()
                .expect("cant take data thread")
                .join()
                .expect("can't join data thread");
            let _ = MAIN_THREAD
                .take()
                .expect("join handle is none")
                .join()
                .expect("can't join thread");
            Ok(())
        }
    }
}
