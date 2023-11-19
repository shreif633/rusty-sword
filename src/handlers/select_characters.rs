use async_trait::async_trait;
use crate::framework::{packet::HandlePacket, world::WorldLock};
use crate::packets::client::select_character::SelectCharacter;

#[async_trait]
impl HandlePacket for SelectCharacter {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        println!("SELECT_CHARACTER {:?}", self);
    }
}