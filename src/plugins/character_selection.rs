use bevy::prelude::*;
use pwhash::unix;
use sqlx::{query_scalar, query};
use crate::components::user::User;
use crate::framework::database::Database;
use crate::repositories::item::{ItemCreateChangeset, create_item};
use crate::repositories::player::{delete_user_player_by_id, find_player_exists_by_name, count_user_players, find_all_deleted_user_players, find_all_user_players, restore_user_player_by_id};
use crate::responses::list_player_deleted_characters::ListPlayerDeletedCharactersResponse;
use crate::responses::character_creation_error::CharacterCreationErrorResponse;
use crate::responses::list_player_characters::ListPlayerCharactersResponse;
use crate::responses::authentication_error::{AuthenticationErrorResponse, Error};
use crate::requests::create_character::{PlayerClass, CreateCharacterRequest};
use crate::requests::restore_deleted_character::RestoreDeletedCharacterRequest;
use crate::requests::delete_character::DeleteCharacterRequest;
use crate::requests::authenticate::AuthenticateRequest;
use super::tcp_server::SocketWriter;

pub struct CharacterSelectionPlugin;

impl Plugin for CharacterSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_authentication);
        app.add_systems(Update, handle_delete_character);
        app.add_systems(Update, handle_restore_deleted_character);
        app.add_systems(Update, handle_create_character);
    }
}

pub fn authenticate(database: &Database, socket_writer: &SocketWriter, username: &str, password: &str) -> Option<u32> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let result = rt.block_on(async move {
        query!("SELECT id, password_hash FROM users WHERE username = ?", username).fetch_one(&database.connection).await
    });
    
    if result.is_err() {
        let authentication_error = AuthenticationErrorResponse { error: Error::WrongPassword };
        socket_writer.write(&mut (&authentication_error).into());
        return None;
    }
    let result = result.unwrap();
    if !unix::verify(password, &result.password_hash) {
        let authentication_error = AuthenticationErrorResponse { error: Error::WrongPassword };
        socket_writer.write(&mut (&authentication_error).into());
        return None;
    }
    let user_id: u32 = result.id.try_into().unwrap();
    Some(user_id)
    // remove already logged user
}

pub fn list_characters(database: &Database, socket_writer: &SocketWriter, user_id: u32) {
    let players = find_all_user_players(database, user_id);
    let list_player_characters = ListPlayerCharactersResponse::new(&players);
    socket_writer.write(&mut (&list_player_characters).into());
}

fn list_deleted_characters(database: &Database, socket_writer: &SocketWriter, user_id: u32) {
    let players = find_all_deleted_user_players(database, user_id);
    let list_player_characters = ListPlayerDeletedCharactersResponse::new(&players);
    socket_writer.write(&mut (&list_player_characters).into());
}

fn create_character(database: &Database, socket_writer: &SocketWriter, user_id: u32, name: &str, base_strength: u8, base_health: u8, base_intelligence: u8, base_wisdom: u8, base_agility: u8, face: u8, hair: u8, class: PlayerClass) -> Option<u32> {
    if count_user_players(database, user_id) >= 5 {
        return None;
    }
    if find_player_exists_by_name(database, name) {
        use crate::responses::character_creation_error::Error;
        let character_creation_error = CharacterCreationErrorResponse { error: Error::NameTaken };
        socket_writer.write(&mut (&character_creation_error).into());
        return None;
    }
    if base_strength + base_health + base_intelligence + base_wisdom + base_agility != 5 {
        return None;
    }
    let base_strength = match class {
        PlayerClass::Knight => 18,
        PlayerClass::Mage => 8,
        PlayerClass::Archer => 14,
    } + base_strength;
    let base_health = match class {
        PlayerClass::Knight => 16,
        PlayerClass::Mage => 10,
        PlayerClass::Archer => 10,
    } + base_health;
    let base_intelligence = match class {
        PlayerClass::Knight => 8,
        PlayerClass::Mage => 18,
        PlayerClass::Archer => 8,
    } + base_intelligence;
    let base_wisdom = match class {
        PlayerClass::Knight => 8,
        PlayerClass::Mage => 16,
        PlayerClass::Archer => 10,
    } + base_wisdom;
    let base_agility = match class {
        PlayerClass::Knight => 10,
        PlayerClass::Mage => 8,
        PlayerClass::Archer => 18,
    } + base_agility;
    let class = match class {
        PlayerClass::Knight => 0,
        PlayerClass::Mage => 1,
        PlayerClass::Archer => 2,
    };
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let player_id = rt.block_on(async move {
        query_scalar!("
        INSERT INTO players 
        (user_id, name, class, specialty, level, base_strength, base_health, base_intelligence, base_wisdom, base_agility, face, hair, x, y, z, weapon_index, shield_index, helmet_index, chest_index, shorts_index, gloves_index, boots_index, current_health_points, maximum_health_points, current_magic_points, maximum_magic_points, experience, rage) 
        values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        ", user_id, name, class, 1, 1, base_strength, base_health, base_intelligence, base_wisdom, base_agility, face, hair, 267701, 242655, 19630, 0, 0, 0, 0, 0, 0, 0, 1000, 2000, 1000, 2000, 0, 0)
        .fetch_one(&database.connection).await.unwrap()
    }) as u32;
    Some(player_id)
}

fn handle_authentication(mut commands: Commands, query: Query<(Entity, &AuthenticateRequest, &SocketWriter)>, database: Res<Database>) {
    for (entity, client_packet, socket_writer) in &query {
        if let Some(user_id) = authenticate(&database, &socket_writer, &client_packet.username, &client_packet.password) {
            commands.entity(entity).insert(User { id: user_id });    
            list_characters(&database, &socket_writer, user_id);
            list_deleted_characters(&database, &socket_writer, user_id);
        }
        commands.entity(entity).remove::<AuthenticateRequest>();
    }
}

fn handle_delete_character(mut commands: Commands, query: Query<(Entity, &User, &DeleteCharacterRequest, &SocketWriter)>, database: Res<Database>) {
    for (entity, user, client_packet, socket_writer) in &query {
        delete_user_player_by_id(&database, user.id, client_packet.character_id);
        list_characters(&database, &socket_writer, user.id);
        list_deleted_characters(&database, &socket_writer, user.id);
        commands.entity(entity).remove::<DeleteCharacterRequest>();
    }
}

fn handle_restore_deleted_character(mut commands: Commands, query: Query<(Entity, &User, &RestoreDeletedCharacterRequest, &SocketWriter)>, database: Res<Database>) {
    for (entity, user, client_packet, socket_writer) in &query {
        restore_user_player_by_id(&database, user.id, client_packet.character_id);
        list_characters(&database, &socket_writer, user.id);
        list_deleted_characters(&database, &socket_writer, user.id);
        commands.entity(entity).remove::<RestoreDeletedCharacterRequest>();
    }
}

fn handle_create_character(mut commands: Commands, query: Query<(Entity, &User, &CreateCharacterRequest, &SocketWriter)>, database: Res<Database>) {
    for (entity, user, client_packet, socket_writer) in &query {
        if let Some(player_id) = create_character(
            &database, 
            &socket_writer, 
            user.id, 
            &client_packet.name, 
            client_packet.base_strength, 
            client_packet.base_health, 
            client_packet.base_intelligence, 
            client_packet.base_wisdom, 
            client_packet.base_agility, 
            client_packet.face, 
            client_packet.hair, 
            client_packet.class
        ) {
            let item_changeset = ItemCreateChangeset { player_id, index: 1, prefix: 0, quantity: 1 };
            create_item(&database, &item_changeset);
            let item_changeset = ItemCreateChangeset { player_id, index: 47, prefix: 0, quantity: 100 };
            create_item(&database, &item_changeset);
        }
        list_characters(&database, &socket_writer, user.id);
        list_deleted_characters(&database, &socket_writer, user.id);
        commands.entity(entity).remove::<CreateCharacterRequest>();
    }
}