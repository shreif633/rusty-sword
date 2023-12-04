use bevy::prelude::*;
use crate::components::user::User;
use crate::configs::player_starter::PlayerStarterConfigs;
use crate::framework::database::Database;
use crate::repositories::item::{ItemCreateChangeset, create_item};
use crate::repositories::player::{delete_user_player_by_id, find_player_exists_by_name, count_user_players, find_all_deleted_user_players, find_all_user_players, restore_user_player_by_id, PlayerCreateChangeset, create_player};
use crate::repositories::user::{find_user_by_username_and_password, UserRow};
use crate::responses::list_player_deleted_characters::ListPlayerDeletedCharactersResponse;
use crate::responses::character_creation_error::CharacterCreationErrorResponse;
use crate::responses::list_player_characters::ListPlayerCharactersResponse;
use crate::responses::authentication_error::{AuthenticationErrorResponse, Error};
use crate::requests::create_character::CreateCharacterRequest;
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

pub fn authenticate(database: &Database, socket_writer: &SocketWriter, username: &str, password: &str) -> Option<UserRow> {
    let user_row = find_user_by_username_and_password(database, username, password);
    if user_row.is_none() {
        let authentication_error = AuthenticationErrorResponse { error: Error::WrongPassword };
        socket_writer.write(&mut (&authentication_error).into());
        return None;
    }
    // remove already logged user
    user_row
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

fn handle_authentication(mut commands: Commands, query: Query<(Entity, &AuthenticateRequest, &SocketWriter)>, database: Res<Database>) {
    for (entity, client_packet, socket_writer) in &query {
        if let Some(user_row) = authenticate(&database, &socket_writer, &client_packet.username, &client_packet.password) {
            commands.entity(entity).insert(User::from(&user_row));    
            list_characters(&database, &socket_writer, user_row.id);
            list_deleted_characters(&database, &socket_writer, user_row.id);
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

fn handle_create_character(mut commands: Commands, query: Query<(Entity, &User, &CreateCharacterRequest, &SocketWriter)>, database: Res<Database>, player_starter_config: Res<PlayerStarterConfigs>) {
    for (entity, user, client_packet, socket_writer) in &query {
        let player_starter_config = player_starter_config.config.get(&client_packet.class).unwrap();
            if count_user_players(&database, user.id) >= 5 {
            return;
        }
        if find_player_exists_by_name(&database, &client_packet.name) {
            use crate::responses::character_creation_error::Error;
            let character_creation_error = CharacterCreationErrorResponse { error: Error::NameTaken };
            socket_writer.write(&mut (&character_creation_error).into());
            return;
        }
        if client_packet.base_strength + client_packet.base_health + client_packet.base_intelligence + client_packet.base_wisdom + client_packet.base_agility != 5 {
            return;
        }
        let changeset = PlayerCreateChangeset {
            user_id: user.id,
            name: &client_packet.name,
            class: client_packet.class,
            experience: player_starter_config.experience.experience,
            base_strength: (player_starter_config.base_points.base_strength + client_packet.base_strength).try_into().unwrap(),
            base_health: (player_starter_config.base_points.base_health + client_packet.base_health).try_into().unwrap(),
            base_intelligence: (player_starter_config.base_points.base_intelligence + client_packet.base_intelligence).try_into().unwrap(),
            base_wisdom: (player_starter_config.base_points.base_wisdom + client_packet.base_wisdom).try_into().unwrap(),
            base_agility: (player_starter_config.base_points.base_agility + client_packet.base_agility).try_into().unwrap(),
            face: client_packet.face,
            hair: client_packet.hair,
            x: player_starter_config.position.x,
            y: player_starter_config.position.y,
            z: player_starter_config.position.z,
            map: player_starter_config.position.map,
        };
        if let Some(player_id) = create_player(&database, &changeset) {
            for item in &player_starter_config.item {
                let item_changeset = ItemCreateChangeset { player_id, index: item.index, prefix: item.prefix, quantity: item.quantity };
                create_item(&database, &item_changeset);
            }
        }
        list_characters(&database, &socket_writer, user.id);
        list_deleted_characters(&database, &socket_writer, user.id);
        commands.entity(entity).remove::<CreateCharacterRequest>();
    }
}