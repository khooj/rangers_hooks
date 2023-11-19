use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use models::{commands::Command, EncodedMessage, SpaceshipInfo};
use ractor::{Actor, ActorProcessingErr, ActorRef, SupervisionEvent};
use tokio::net::{TcpSocket, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message as TungMessage, WebSocketStream};
use tokio_util::sync::CancellationToken;

use crate::{
    commands::AbsolutePoint,
    main_thread::{MainActorMessage, MAIN_ACTOR_NAME},
};

pub struct WebsocketsActor;

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    PlayerInfo(SpaceshipInfo),
    NewConn(ActorRef<ConnectionMessage>),
}

pub struct State {
    token: CancellationToken,
    conns: Vec<ActorRef<ConnectionMessage>>,
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
        println!("starting ws");
        let token = CancellationToken::new();
        let cloned_token = token.clone();
        let hndl = tokio::runtime::Handle::current();
        hndl.spawn(async move {
            let addr = "127.0.0.1:3012";
            let sock = TcpSocket::new_v4().unwrap();
            sock.set_reuseaddr(true).unwrap();
            sock.bind(addr.parse().unwrap()).unwrap();
            let listener = sock.listen(8).unwrap();

            let f = async {
                while let Ok((stream, _)) = listener.accept().await {
                    let (actor, _) = Actor::spawn_linked(
                        None,
                        WebsocketConnectionActor,
                        stream,
                        myself.clone().into(),
                    )
                    .await
                    .expect("can't spawn ws conn actor");
                    myself.cast(Message::NewConn(actor)).unwrap();
                }
            };

            tokio::select! {
                _ = f => {},
                _ = token.cancelled() => {},
            };
            println!("stopped ws actor");
        });
        Ok(State {
            token: cloned_token,
            conns: vec![],
        })
    }

    async fn post_stop(
        &self,
        _myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        println!("requested to stop ws actor");
        state.token.cancel();
        Ok(())
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let m = match msg {
            Message::PlayerInfo(info) => EncodedMessage::PlayerInfo(info),
            Message::NewConn(conn) => {
                state.conns.push(conn);
                return Ok(());
            }
        };
        let m = bincode::serialize(&m)?;
        for conn in &state.conns {
            conn.cast(ConnectionMessage::Msg(m.clone()))?;
        }
        Ok(())
    }

    async fn handle_supervisor_evt(
        &self,
        _myself: ActorRef<Self::Msg>,
        msg: SupervisionEvent,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        if let SupervisionEvent::ActorTerminated(cell, ..) = msg {
            let el = state
                .conns
                .iter()
                .position(|el| el.get_id() == cell.get_id());
            if let Some(idx) = el {
                state.conns.remove(idx);
            }
        }
        Ok(())
    }
}

struct WebsocketConnectionActor;

#[derive(Debug)]
pub enum ConnectionMessage {
    Msg(Vec<u8>),
    Sender(SplitSink<WebSocketStream<TcpStream>, TungMessage>),
    RecvMsg(Vec<u8>),
    ReceivingStopped,
}

pub struct WebsocketConnectionState {
    ws_sender: Option<SplitSink<WebSocketStream<TcpStream>, TungMessage>>,
    token: CancellationToken,
}

#[async_trait::async_trait]
impl Actor for WebsocketConnectionActor {
    type Msg = ConnectionMessage;
    type State = WebsocketConnectionState;
    type Arguments = TcpStream;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        arg: TcpStream,
    ) -> Result<Self::State, ActorProcessingErr> {
        let token = CancellationToken::new();
        let cloned_token = token.clone();
        tokio::spawn(async move {
            let ws_stream = accept_async(arg).await.expect("can't accept ws");
            let (ws_sender, mut ws_recv) = ws_stream.split();
            myself.cast(ConnectionMessage::Sender(ws_sender)).unwrap();
            loop {
                tokio::select! {
                    msg = ws_recv.next() => {
                        match msg {
                            Some(msg) => {
                                let msg = match msg {
                                    Ok(k) => k,
                                    Err(err) => { println!("error unpacking ws msg: {}", err); break; },
                                };
                                #[allow(clippy::single_match)]
                                match msg {
                                    // TungMessage::Close(_) => break,
                                    TungMessage::Binary(m) => {
                                        println!("got binary message");
                                        myself.cast(ConnectionMessage::RecvMsg(m)).unwrap();
                                    }
                                    _ => {}
                                }
                            },
                            None => {
                                // println!("got none msg");
                                break
                            }
                        }
                    },
                    _ = token.cancelled() => {
                        println!("ws conn token canceled");
                        break
                    }
                }
            }
            println!("stopped ws conn actor worker");
            myself.cast(ConnectionMessage::ReceivingStopped).unwrap();
        });
        Ok(WebsocketConnectionState {
            ws_sender: None,
            token: cloned_token,
        })
    }

    async fn post_stop(
        &self,
        _myself: ActorRef<Self::Msg>,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        println!("requested to stop ws conn");
        state.token.cancel();
        Ok(())
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        msg: ConnectionMessage,
        state: &mut WebsocketConnectionState,
    ) -> Result<(), ActorProcessingErr> {
        match msg {
            ConnectionMessage::Sender(s) => state.ws_sender = Some(s),
            ConnectionMessage::Msg(m) => {
                state
                    .ws_sender
                    .as_mut()
                    .unwrap()
                    .send(TungMessage::Binary(m))
                    .await?;
            }
            ConnectionMessage::RecvMsg(m) => {
                let data: Command = bincode::deserialize(&m).expect("can't deserialize");

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
                        myself.stop(None);
                    }
                }
            }
            ConnectionMessage::ReceivingStopped => {
                myself.stop(None);
            }
        };
        Ok(())
    }
}
