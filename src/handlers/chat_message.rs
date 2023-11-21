use async_trait::async_trait;
use crate::framework::packet::HandlePacket;
use crate::framework::player::PlayerManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::packets::client::chat_message::ChatMessage;

#[async_trait]
impl HandlePacket for ChatMessage {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        let character_name = {
            let player = world.get_player_by_id(user_id).unwrap();
            let player = player.read().unwrap();
            player.name.clone()
        };
        let chat_message = crate::packets::server::chat_message::ChatMessage { 
            character_name, 
            message: self.message.clone()
        };
        for other_player in world.get_players_in_sight(user_id) {
            other_player.send(&mut (&chat_message).into()).await;
        }
    }
}