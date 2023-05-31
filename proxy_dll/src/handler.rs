use models::commands::Command;
use spmc::{Receiver, TryRecvError};
use std::{
    ffi::c_void,
    sync::atomic::{AtomicU8, Ordering},
};
use windows::Win32::{
    Foundation::{CloseHandle, HINSTANCE},
    System::{
        LibraryLoader::FreeLibraryAndExitThread,
        Threading::{CreateThread, ExitThread, Sleep, SleepEx, THREAD_CREATION_FLAGS},
    },
};
use ws::{util::Token, Message, Sender};

use crate::{
    commands::AbsolutePoint,
    main_thread::{MainThread, STOP_FLAG},
};

const CHECK_EVENT: Token = Token(101);
const SHUTDOWN_EVENT: Token = Token(102);

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
        // self.sender.shutdown().unwrap();
        println!("closing");
    }

    fn on_timeout(&mut self, event: Token) -> ws::Result<()> {
        match event {
            CHECK_EVENT => {
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
                self.sender.timeout(100, CHECK_EVENT)
            }
            SHUTDOWN_EVENT => self.sender.shutdown(),
            _ => Ok(()),
        }
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        if let ws::Message::Binary(data) = msg {
            let data: Command = bincode::deserialize(&data).expect("can't deserialize");

            match data {
                Command::MouseLeftClick { x, y } => {
                    println!("mouse left click command: {} {}", x, y);
                    unsafe {
                        super::commands::mouse_left_click(AbsolutePoint { x, y });
                    }
                }
                // https://stackoverflow.com/questions/54850877/unload-dll-from-process
                Command::DetachLibrary => unsafe {
                    let hndl = CreateThread(
                        None,
                        0,
                        Some(detach_library),
                        Some(self.instance.0 as *const c_void),
                        THREAD_CREATION_FLAGS(0),
                        None,
                    )
                    .unwrap();
                    CloseHandle(hndl);
                    return self.sender.close(ws::CloseCode::Normal);
                },
            }
        }
        Ok(())
    }
}

unsafe extern "system" fn detach_library(module: *mut c_void) -> u32 {
    STOP_FLAG.store(1, Ordering::SeqCst);
    Sleep(500);
    let module = HINSTANCE(module as isize);
    MainThread::stop().expect("can't stop main thread");
    Sleep(5000);
    println!("second thread");
    FreeLibraryAndExitThread(module, 0);
    // ExitThread(0);
}

pub unsafe extern "system" fn start_detach_library(module: HINSTANCE) {
    let hndl = CreateThread(
        None,
        0,
        Some(detach_library),
        Some(module.0 as *const c_void),
        THREAD_CREATION_FLAGS(0),
        None,
    )
    .unwrap();
    CloseHandle(hndl);
}
