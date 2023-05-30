use crate::handler::{Handler, HANDLERS_COUNT};

use super::player::*;
use models::EncodedMessage;
use std::{
    sync::atomic::{AtomicU8, Ordering},
    sync::{Arc, Mutex},
    thread::{self, sleep, JoinHandle},
    time::Duration,
};
use thiserror::Error;
use windows::Win32::Foundation::HINSTANCE;
use ws::{Builder, Sender, Settings};

#[derive(Error, Debug)]
pub enum MainThreadError {}

pub struct MainThread {}

static mut MAIN_THREAD: Option<JoinHandle<()>> = None;
static mut DATA_THREAD: Option<JoinHandle<()>> = None;
const DATA_THREAD_SLEEP: Duration = Duration::from_millis(100);
pub static STOP_FLAG: AtomicU8 = AtomicU8::new(0);
static mut W_SENDER: Option<Sender> = None;

impl MainThread {
    pub fn start(module: HINSTANCE) -> Result<(), MainThreadError> {
        let (mut tx, rx) = spmc::channel();
        let mut s = Settings::default();
        s.max_connections = 1;
        s.panic_on_capacity = true;
        let w = Builder::new()
            .with_settings(s)
            .build(move |out| Handler::new(out, rx.clone(), module))
            .expect("can't create ws");
        // let w = Arc::new(Mutex::new(w));
        let w_sender = w.broadcaster();
        unsafe {
            W_SENDER = Some(w_sender);
        }

        let hndl = thread::Builder::new()
            .name("proxy_dll-ws".into())
            .spawn(move || {
                w.listen("127.0.0.1:3012").expect("can't start ws");
                println!("main thread close");
            })
            .unwrap();
        let hndl = Some(hndl);

        let hndl2 = thread::Builder::new()
            .name("proxy_dll-data".into())
            .spawn(move || {
                let mut player_info = None;

                loop {
                    if STOP_FLAG.load(Ordering::SeqCst) != 0 {
                        println!("data thread close");
                        return;
                    }

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
            })
            .unwrap();
        let hndl2 = Some(hndl2);
        unsafe {
            MAIN_THREAD = hndl;
            DATA_THREAD = hndl2;
        }
        Ok(())
    }

    // dont sure if i need to manually stop thread
    pub fn stop() -> Result<(), MainThreadError> {
        unsafe {
            STOP_FLAG.store(1, Ordering::SeqCst);
            W_SENDER
                .take()
                .expect("cant take w_sender")
                .shutdown()
                .expect("can't shutdown websocket");
            let _ = DATA_THREAD
                .take()
                .expect("cant take data thread")
                .join()
                .expect("can't join data thread");
            // let _ = MAIN_THREAD
            //     .take()
            //     .expect("join handle is none")
            //     .join()
            //     .expect("can't join thread");
            Ok(())
        }
    }
}
