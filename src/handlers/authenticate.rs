use async_trait::async_trait;
use crate::framework::packet::HandlePacket;
use crate::framework::user::UserManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::packets::client::authenticate::Authenticate;
use crate::packets::server::authentication_error::{AuthenticationError, Error};
use crate::packets::server::list_player_deleted_characters::{PlayerDeletedCharacter, ListPlayerDeletedCharacters};
use crate::packets::server::list_player_characters::{ListPlayerCharacters, PlayerCharacter};

#[async_trait]
impl HandlePacket for Authenticate {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        let current_user_lock = world.get_user_lock_by_id(user_id).unwrap();

        if self.password == "admin" && self.username == "admin" {
            let mut characters: Vec<PlayerCharacter> = vec![];
            let items_indexes: Vec<u16> = vec![261, 262, 263, 264, 265, 338, 781, 4336, 5007, 5017];
            let character = PlayerCharacter { 
                id: user_id, 
                name: format!("Hermit{}", user_id).into(), 
                class: 1, 
                specialty: 7, 
                level: 60, 
                unknown1: vec![0, 0, 0, 0],
                strength: 8, 
                health: 94, 
                intelligence: 92, 
                wisdom: 50, 
                agility: 8, 
                face: 6, 
                hair: 6,
                items_indexes 
            };
            characters.push(character);
            let list_player_characters = ListPlayerCharacters { unknown1: vec![0, 0, 0, 0, 0], characters };
            current_user_lock.send(&mut (&list_player_characters).into()).await;
            
            let mut characters: Vec<PlayerDeletedCharacter> = vec![];
            let character = PlayerDeletedCharacter { 
                id: 574, 
                name: "DeletedChar".to_owned(), 
                level: 69, 
                remaining_days: 8,
                class: 1, 
            };
            characters.push(character);
            let list_player_characters = ListPlayerDeletedCharacters { characters };
            current_user_lock.send(&mut (&list_player_characters).into()).await;

        } else if self.password == "already" && self.username == "admin" {
            let authentication_error = AuthenticationError { error: Error::AlreadyLogged };
            current_user_lock.send(&mut (&authentication_error).into()).await;

        } else {
            let authentication_error = AuthenticationError { error: Error::WrongPassword };
            current_user_lock.send(&mut (&authentication_error).into()).await;
        }
    }
}