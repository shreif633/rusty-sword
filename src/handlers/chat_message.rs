use async_trait::async_trait;
use crate::framework::packet::HandlePacket;
use crate::framework::user::UserManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::packets::client::chat_message::{ChatMessage, self};

#[async_trait]
impl HandlePacket for ChatMessage {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        let current_user_lock = world.get_user_lock_by_id(user_id).unwrap();

        let chat_message = crate::packets::server::chat_message::ChatMessage { 
            character_name: "Hermit".to_string(), 
            message: self.message.clone()
        };
        current_user_lock.send(&mut (&chat_message).into()).await;
    }
}