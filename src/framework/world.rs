use std::{sync::{Arc, RwLock}, collections::HashMap};
use tokio::sync::mpsc::Sender;

use crate::framework::user::{User, UserLock};

#[derive(Debug)]
pub struct World {
    last_id: u32,
    players: Arc<RwLock<HashMap<u32, UserLock>>>
}

pub type WorldLock = Arc<RwLock<World>>;

pub trait WorldManager {
    fn default() -> Self;
    fn insert_user(&self, writer: Sender<Vec<u8>>) -> u32;
    fn get_user_lock_by_id(&self, player_id: u32) -> Option<UserLock>;
}

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

}