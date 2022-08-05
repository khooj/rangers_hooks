use std::{thread::{self, JoinHandle}};
use super::player::*;

#[derive(Debug)]
pub enum MainThreadError {
}

impl std::fmt::Display for MainThreadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct MainThread {}

static mut MAIN_THREAD: Option<JoinHandle<()>> = None;

impl MainThread {
    pub fn start() -> Result<(), MainThreadError> {
        // SAFETY: no strong guarantees about concurrent start/stop
        // it is supposed to be used only in DllMain
        unsafe {
            MAIN_THREAD = Some(thread::spawn(|| {
                loop {
                if let Some(p) = get_player_struct() {
                    (*p).name();
                    (*p).experience();
                }

                thread::sleep(std::time::Duration::from_millis(100));

                }
            }));
        }
        Ok(())
    }

    pub fn stop() -> Result<(), MainThreadError> {
        unsafe {
            let _ = MAIN_THREAD.take().unwrap().join().expect("can't join thread");
            MAIN_THREAD = None;
            Ok(())
        }
    }
}