use async_trait::async_trait;
use crate::framework::packet::HandlePacket;
use crate::framework::user::UserManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::packets::client::chat_message::ChatMessage;
use crate::packets::server::player_appear::{PlayerAppear, PlayerClass};

#[async_trait]
impl HandlePacket for ChatMessage {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        let current_user_lock = world.get_user_lock_by_id(user_id).unwrap();

        let chat_message = crate::packets::server::chat_message::ChatMessage { 
            character_name: format!("Hermit{}", user_id).into(), 
            message: self.message.clone()
        };
        // current_user_lock.send(&mut (&chat_message).into()).await;

        // let player_appear = PlayerAppear { 
        //     player_id: 2, 
        //     name: format!("Hermit2").into(), 
        //     class: PlayerClass::Mage, 
        //     is_current_player: false,
        //     x: 267701, 
        //     y: 242655, 
        //     z: 19630, 
        //     unknown1: vec![1, 0, 0, 0, 0, 136, 0, 0, 0, 0], 
        //     weapon_index: 781, 
        //     shield_index: 0, 
        //     helmet_index: 262, 
        //     chest_index: 261, 
        //     shorts_index: 265, 
        //     gloves_index: 263, 
        //     boots_index: 264, 
        //     unknown2: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
        //     face: 6, 
        //     hair: 6, 
        //     unknown3: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 2, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] 
        // };
        // current_user_lock.send(&mut (&player_appear).into()).await;

        world.send(&mut (&chat_message).into()).await;
    }
}