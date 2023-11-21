use std::{sync::{Arc, RwLock}, collections::HashMap};
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;
use crate::framework::user::{User, UserLock};
use super::player::{PlayerLock, Player};

#[derive(Debug)]
pub struct World {
    last_id: u32,
    users: Arc<RwLock<HashMap<u32, UserLock>>>,
    players: Arc<RwLock<HashMap<u32, PlayerLock>>>,
}

pub type WorldLock = Arc<RwLock<World>>;

#[async_trait]
pub trait WorldManager {
    fn default() -> Self;

    fn insert_user(&self, writer: Sender<Vec<u8>>) -> u32;
    fn remove_user(&self, player_id: u32);
    fn get_user_by_id(&self, player_id: u32) -> Option<UserLock>;

    fn create_player(&self, user_id: u32) -> PlayerLock;
    fn get_player_by_id(&self, player_id: u32) -> Option<PlayerLock>;
    fn get_other_players_in_sight(&self, player_id: u32) -> Vec<PlayerLock>;
    fn get_players_in_sight(&self, player_id: u32) -> Vec<PlayerLock>;
}

#[async_trait]
impl WorldManager for WorldLock {

    fn default() -> Self {
        Arc::new(RwLock::new(World { 
            last_id: 0,
            users: Arc::new(RwLock::new(HashMap::<u32, UserLock>::new())),
            players: Arc::new(RwLock::new(HashMap::<u32, PlayerLock>::new()))
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
            let mut users = world.users.write().unwrap();
            let user_lock = Arc::new(RwLock::new(User { id, writer }));
            users.insert(id, user_lock);
        }
        id
    }

    fn create_player(&self, player_id: u32) -> PlayerLock {
        let world = self.read().unwrap();
        let mut players = world.players.write().unwrap();
        let writer = {
            let get_user_by_id = self.get_user_by_id(player_id).unwrap();
            let user = get_user_by_id.read().unwrap();
            user.writer.clone()
        };
        let player_lock = Arc::new(RwLock::new(Player {
            id: player_id,
            writer,
            x: 0,
            y: 0,
            z: 0,
            name: "".to_string(), 
            face: 0, 
            hair: 0, 
            weapon_index: 0, 
            shield_index: 0, 
            helmet_index: 0, 
            chest_index: 0, 
            shorts_index: 0, 
            gloves_index: 0, 
            boots_index: 0
        }));
        players.insert(player_id, player_lock.clone());
        player_lock.clone()
    }

    fn remove_user(&self, player_id: u32) {
        let world = self.read().unwrap();
        let mut players = world.players.write().unwrap();
        players.remove(&player_id);
    }

    fn get_user_by_id(&self, user_id: u32) -> Option<UserLock> {
        let world = self.read().unwrap();
        let users = world.users.read().unwrap();
        users.get(&user_id).cloned()
    }

    fn get_player_by_id(&self, player_id: u32) -> Option<PlayerLock> {
        let world = self.read().unwrap();
        let players = world.players.read().unwrap();
        players.get(&player_id).cloned()
    }

    fn get_players_in_sight(&self, _player_id: u32) -> Vec<PlayerLock> {
        let world = self.read().unwrap();
        let players = world.players.write().unwrap();
        players.clone().values().cloned().collect::<Vec<PlayerLock>>()
    }

    fn get_other_players_in_sight(&self, player_id: u32) -> Vec<PlayerLock> {
        let world = self.read().unwrap();
        let players = world.players.write().unwrap();
        let mut players = players.clone().values().cloned().collect::<Vec<PlayerLock>>();
        players.retain(|player| {
            let player = player.read().unwrap();
            player.id != player_id
        });
        players
    }

}