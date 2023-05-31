use crate::{
    handler::start_detach_library, websockets::WebsocketsActor, world_data::WorldDataActor,
};

use ractor::{Actor, ActorProcessingErr, ActorRef};
use std::thread::{self, JoinHandle};
use thiserror::Error;
use windows::Win32::Foundation::HINSTANCE;

#[derive(Error, Debug)]
pub enum MainThreadError {}

pub struct MainThread {}

static mut MAIN_THREAD: Option<JoinHandle<()>> = None;

pub const MAIN_ACTOR_NAME: &str = "main_actor";

pub struct MainActor;

pub struct MainActorState {
    module: HINSTANCE,
}

pub enum MainActorMessage {
    DetachLibrary,
}

#[async_trait::async_trait]
impl Actor for MainActor {
    type Msg = MainActorMessage;
    type State = MainActorState;
    type Arguments = HINSTANCE;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        module: HINSTANCE,
    ) -> Result<Self::State, ActorProcessingErr> {
        let (ws_actor, _) = Actor::spawn_linked(None, WebsocketsActor, (), myself.clone().into())
            .await
            .expect("can't start websockets actor");
        let (_, _) = Actor::spawn_linked(
            None,
            WorldDataActor,
            ws_actor.clone(),
            myself.clone().into(),
        )
        .await
        .expect("can't start world data actor");
        Ok(MainActorState { module })
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            MainActorMessage::DetachLibrary => {
                println!("cmd to detach lib");
                myself.stop(None);
                unsafe {
                    start_detach_library(state.module);
                }
            }
        };
        Ok(())
    }
}

impl MainThread {
    pub fn start(module: HINSTANCE) -> Result<(), MainThreadError> {
        let hndl = thread::Builder::new()
            .name("proxy_dll-main".into())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .worker_threads(4)
                    .build()
                    .expect("can't build tokio runtime");

                rt.block_on(async {
                    let (_, handle) =
                        Actor::spawn(Some(MAIN_ACTOR_NAME.to_string()), MainActor, module)
                            .await
                            .expect("can't create main actor");
                    println!("registered actors: {:?}", ractor::registry::registered());
                    handle.await.expect("can't wait for main actor");
                });
                println!("main thread close");
            })
            .unwrap();
        let hndl = Some(hndl);
        unsafe {
            MAIN_THREAD = hndl;
        }
        Ok(())
    }

    pub fn stop() -> Result<(), MainThreadError> {
        unsafe {
            MAIN_THREAD
                .take()
                .expect("join handle is none")
                .join()
                .expect("can't join thread");
            Ok(())
        }
    }
}
