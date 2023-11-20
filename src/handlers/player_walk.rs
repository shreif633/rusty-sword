use async_trait::async_trait;
use crate::framework::packet::HandlePacket;
use crate::framework::user::UserManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::packets::client::player_walk::PlayerWalk;

#[async_trait]
impl HandlePacket for PlayerWalk {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        { 
            let other_users_ids = world.get_other_users_ids_around_id(user_id);
            for other_id in other_users_ids {
                let player_walk = crate::packets::server::player_walk::PlayerWalk { 
                    player_id: user_id, 
                    delta_x: self.delta_x, 
                    delta_y: self.delta_y, 
                    delta_z: self.delta_z 
                };
                let other_user_lock = world.get_user_lock_by_id(other_id).unwrap();
                other_user_lock.send(&mut (&player_walk).into()).await;
            }
        }
    }
}