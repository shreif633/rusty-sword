use async_trait::async_trait;
use crate::framework::packet::HandlePacket;
use crate::framework::player::PlayerManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::packets::client::emote::Emote;

#[async_trait]
impl HandlePacket for Emote {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        let emote = crate::packets::server::emote::Emote { 
            player_id: user_id, 
            emote_index: self.emote_index
        };
        for other_player in world.get_players_in_sight(user_id) {
            other_player.send(&mut (&emote).into()).await;
        }
    }
}