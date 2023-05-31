use crate::websockets::Message as WebsocketMessage;

use super::player::get_player_struct;
use models::SpaceshipInfo;
use ractor::{cast, concurrency::Duration, Actor, ActorProcessingErr, ActorRef};

pub struct WorldDataActor;

#[derive(Debug, Clone)]
pub enum Message {
    Event,
}

pub struct WorldDataActorState {
    player_info: Option<SpaceshipInfo>,
    parent: ActorRef<WebsocketMessage>,
}

#[async_trait::async_trait]
impl Actor for WorldDataActor {
    type Msg = Message;
    type State = WorldDataActorState;
    type Arguments = ActorRef<WebsocketMessage>;

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        parent: ActorRef<WebsocketMessage>,
    ) -> Result<Self::State, ActorProcessingErr> {
        myself.send_interval(Duration::from_millis(100), || Message::Event);
        Ok(WorldDataActorState {
            player_info: None,
            parent,
        })
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let new_player_info = get_player_struct();
        if new_player_info != state.player_info {
            state.player_info = new_player_info;
            cast!(
                state.parent,
                WebsocketMessage::PlayerInfo(state.player_info.as_ref().unwrap().clone())
            )?;
        }
        Ok(())
    }
}
