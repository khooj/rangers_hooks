use crate::handler::{Handler, HANDLERS_COUNT};

use super::player::*;
use models::EncodedMessage;
use std::{
    sync::atomic::Ordering,
    thread::{self, sleep, JoinHandle},
    time::Duration,
};
use thiserror::Error;
use ws::{Builder, Settings};

#[derive(Error, Debug)]
pub enum MainThreadError {}

pub struct MainThread {}

static mut MAIN_THREAD: Option<JoinHandle<()>> = None;
static mut DATA_THREAD: Option<JoinHandle<()>> = None;
const DATA_THREAD_SLEEP: Duration = Duration::from_millis(100);

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

        let hndl2 = Some(thread::spawn(move || {
            let mut player_info = None;

            loop {
                if HANDLERS_COUNT.load(Ordering::SeqCst) == 0 {
                    sleep(DATA_THREAD_SLEEP);
                    player_info = None;
                    continue;
                }

                let msg = {
                    let new_player_info = get_player_struct();
                    if new_player_info != player_info {
                        player_info = new_player_info;
                        let m = match &player_info {
                            Some(e) => EncodedMessage::PlayerInfo(e.clone()),
                            None => EncodedMessage::EmptyPlayerInfo,
                        };
                        m
                    } else {
                        EncodedMessage::NoMessage
                    }
                };

                match msg {
                    EncodedMessage::NoMessage => {}
                    m => {
                        let m = match bincode::serialize(&m) {
                            Ok(k) => k,
                            Err(e) => {
                                eprintln!("can't serialize message: {}", e);
                                sleep(DATA_THREAD_SLEEP);
                                continue;
                            }
                        };
                        // ignore SendError if no receivers exist, we are ok
                        let _ = tx.send(m);
                    }
                };

                sleep(DATA_THREAD_SLEEP);
            }
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
