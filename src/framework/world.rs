use std::{sync::{Arc, RwLock}, collections::HashMap};
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use crate::framework::user::{User, UserLock};

use super::{packet::Packet, user::UserManager};

#[derive(Debug)]
pub struct World {
    last_id: u32,
    players: Arc<RwLock<HashMap<u32, UserLock>>>
}

pub type WorldLock = Arc<RwLock<World>>;

#[async_trait]
pub trait WorldManager {
    fn default() -> Self;
    fn insert_user(&self, writer: Sender<Vec<u8>>) -> u32;
    fn get_user_lock_by_id(&self, player_id: u32) -> Option<UserLock>;
    async fn send(&self, packet: &mut Packet);
    fn get_other_users_ids_around_id(&self, player_id: u32) -> Vec<u32>;
    fn get_users_ids_around_id(&self, player_id: u32) -> Vec<u32>;
}

#[async_trait]
impl WorldManager for WorldLock {

    fn default() -> Self {
        Arc::new(RwLock::new(World { 
            last_id: 0,
            players: Arc::new(RwLock::new(HashMap::<u32, UserLock>::new()))
        }))
    }

    fn insert_user(&self, writer: Sender<Vec<u8>>) -> u32 {
        let id = {
            let mut world = self.write().unwrap();
            world.last_id += 1;
            world.last_id
        };
        let world = self.read().unwrap();
        {
            let mut players = world.players.write().unwrap();
            let user_lock = Arc::new(RwLock::new(User { id, character: None, writer }));
            players.insert(id, user_lock);
        }
        id
    }

    fn get_user_lock_by_id(&self, player_id: u32) -> Option<UserLock> {
        let world = self.read().unwrap();
        let players = world.players.read().unwrap();
        players.get(&player_id).cloned()
    }

    async fn send(&self, packet: &mut Packet) {
        let ids = {
            let world = self.read().unwrap();
            let players = world.players.write().unwrap();
            players.clone().into_keys().collect::<Vec<u32>>()
        };
        for id in ids {
            let user = self.get_user_lock_by_id(id).unwrap();
            user.send(packet).await;
        }
    }

    fn get_other_users_ids_around_id(&self, player_id: u32) -> Vec<u32> {
        let world = self.read().unwrap();
        let players = world.players.write().unwrap();
        let mut ids = players.clone().keys().copied().collect::<Vec<u32>>();
        ids.retain(|id| *id != player_id);
        ids
    }

    fn get_users_ids_around_id(&self, player_id: u32) -> Vec<u32> {
        let world = self.read().unwrap();
        let players = world.players.write().unwrap();
        players.clone().keys().copied().collect::<Vec<u32>>()
    }

}