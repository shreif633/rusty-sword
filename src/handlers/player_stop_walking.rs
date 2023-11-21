use async_trait::async_trait;
use crate::framework::packet::HandlePacket;
use crate::framework::player::PlayerManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::packets::client::player_stop_walking::PlayerStopWalking;

#[async_trait]
impl HandlePacket for PlayerStopWalking {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        {
            let player = world.get_player_by_id(user_id).unwrap();
            let mut player = player.write().unwrap();

            let delta_x: u32 = self.delta_x.try_into().unwrap();
            if self.delta_x > 128 { 
                player.x = player.x - (256 - delta_x);
            } else { 
                player.x = player.x + delta_x;
            }

            let delta_y: u32 = self.delta_y.try_into().unwrap();
            if self.delta_y > 128 { 
                player.y = player.y - (256 - delta_y);
            } else { 
                player.y = player.y + delta_y;
            }

            let delta_z: u32 = self.delta_z.try_into().unwrap();
            if self.delta_z > 128 { 
                player.z = player.z - (256 - delta_z);
            } else { 
                player.z = player.z + delta_z;
            }
        }
        let player_stop_walking = crate::packets::server::player_stop_walking::PlayerStopWalking { 
            player_id: user_id, 
            delta_x: self.delta_x, 
            delta_y: self.delta_y, 
            delta_z: self.delta_z 
        };
        for other_player in world.get_other_players_in_sight(user_id) {
            other_player.send(&mut (&player_stop_walking).into()).await;
        }
    }
}