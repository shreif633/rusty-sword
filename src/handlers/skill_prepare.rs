use std::time::Duration;
use async_trait::async_trait;
use tokio::time::sleep;
use crate::framework::packet::HandlePacket;
use crate::framework::user::UserManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::packets::client::skill_prepare::SkillPrepare;
use crate::packets::server::skill_execute::{SkillExecute, TargetType};

#[async_trait]
impl HandlePacket for SkillPrepare {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        // let current_user_lock = world.get_user_lock_by_id(user_id).unwrap();

        let other_users_ids = world.get_users_ids_around_id(user_id);
        for other_id in other_users_ids {
            let skill_prepare = crate::packets::server::skill_prepare::SkillPrepare { 
                player_id: user_id, 
                unknown: 5, // 9 makes you dead - 5 splash - 6 evades?
                skill_index: Some(self.skill_index), 
                target_id: Some(self.target_id)
            };
            let other_user_lock = world.get_user_lock_by_id(other_id).unwrap();
            other_user_lock.send(&mut (&skill_prepare).into()).await;
        }
        
        sleep(Duration::from_millis(1000)).await;

        let other_users_ids = world.get_users_ids_around_id(user_id);
        for other_id in other_users_ids {
            let skill_execute = SkillExecute { 
                skill_index: self.skill_index, 
                player_id: user_id, 
                target_id: self.target_id, 
                target_type: TargetType::Player, 
                unknown: 5, 
                normal_damage: None, 
                explosive_blow_damage: None, 
                damage_type: None, 
                soul_pocket_damage: None 
            };
            let other_user_lock = world.get_user_lock_by_id(other_id).unwrap();
            other_user_lock.send(&mut (&skill_execute).into()).await;
        }
    }
}