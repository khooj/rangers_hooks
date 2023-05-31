use models::{commands::Command, EncodedMessage, SpaceshipInfo};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use ws::{Builder, Sender, Settings, WebSocket};

use crate::{
    commands::AbsolutePoint,
    main_thread::{MainActorMessage, MAIN_ACTOR_NAME},
};

pub struct WebsocketsActor;

#[derive(Debug, Clone)]
pub enum Message {
    PlayerInfo(SpaceshipInfo),
}

struct WsFac(ActorRef<Message>);

impl ws::Factory for WsFac {
    type Handler = WsConnWrapper;

    fn connection_made(&mut self, out: Sender) -> Self::Handler {
        let hndl = tokio::runtime::Handle::current();
        hndl.block_on(async {
            let actor = WebsocketsActor::conn_actor(self.0.clone())
                .await
                .expect("can't create new ws actor");
            WsConnWrapper(actor, out)
        })
    }
}

pub struct State {
    websocket: WebSocket<WsFac>,
}

#[async_trait::async_trait]
impl Actor for WebsocketsActor {
    type Msg = Message;
    type State = State;
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        let mut s = Settings::default();
        s.max_connections = 1;
        s.panic_on_capacity = true;
        let w = Builder::new()
            .with_settings(s)
            .build(WsFac(myself.clone()))
            .expect("can't create ws");
        Ok(State { websocket: w })
    }

    async fn post_stop(
        &self,
        myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        state.websocket.broadcaster().shutdown()?;
        Ok(())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let m = match msg {
            Message::PlayerInfo(info) => EncodedMessage::PlayerInfo(info),
        };
        let m = bincode::serialize(&m)?;
        state
            .websocket
            .broadcaster()
            .broadcast(ws::Message::Binary(m))?;
        Ok(())
    }
}

impl WebsocketsActor {
    async fn conn_actor(
        myself: ActorRef<Message>,
    ) -> Result<ActorRef<ConnectionMessage>, ActorProcessingErr> {
        let (act, _) =
            Actor::spawn_linked(None, WebsocketConnectionActor, (), myself.clone().into()).await?;
        Ok(act)
    }
}

struct WebsocketConnectionActor;

#[derive(Debug, Clone)]
enum ConnectionMessage {}

#[async_trait::async_trait]
impl Actor for WebsocketConnectionActor {
    type Msg = ConnectionMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        _: (),
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(())
    }
}

struct WsConnWrapper(ActorRef<ConnectionMessage>, Sender);

impl ws::Handler for WsConnWrapper {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        if let ws::Message::Binary(data) = msg {
            let data: Command = bincode::deserialize(&data).expect("can't deserialize");

            match data {
                Command::MouseLeftClick { x, y } => {
                    println!("mouse left click command: {} {}", x, y);
                    unsafe {
                        super::commands::mouse_left_click(AbsolutePoint { x, y });
                    }
                }
                Command::DetachLibrary => {
                    let main_actor = ractor::registry::where_is(MAIN_ACTOR_NAME.to_string());
                    let main_actor = main_actor.unwrap();
                    main_actor
                        .send_message(MainActorMessage::DetachLibrary)
                        .unwrap();
                    return self.1.close(ws::CloseCode::Normal);
                }
            }
        }
        Ok(())
    }
}
