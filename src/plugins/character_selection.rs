use bevy::prelude::*;
use pwhash::unix;
use sqlx::{query, query_scalar, types::chrono::Local};
use crate::{packets::{client::{authenticate::Authenticate, delete_character::DeleteCharacter, restore_deleted_character::RestoreDeletedCharacter, create_character::{PlayerClass, CreateCharacter}}, server::{authentication_error::{AuthenticationError, Error}, list_player_characters::{PlayerCharacter, ListPlayerCharacters}, list_player_deleted_characters::{PlayerDeletedCharacter, ListPlayerDeletedCharacters}, character_creation_error::CharacterCreationError}}, framework::database::Database};
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

#[derive(Component)]
pub struct User {
    pub id: u32,
}

pub fn authenticate(database: &Database, socket_writer: &SocketWriter, username: &str, password: &str) -> Option<u32> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let result = rt.block_on(async move {
        query!("SELECT id, password_hash FROM users WHERE username = ?", username).fetch_one(&database.connection).await
    });
    
    if result.is_err() {
        let authentication_error = AuthenticationError { error: Error::WrongPassword };
        socket_writer.write(&mut (&authentication_error).into());
        return None;
    }
    let result = result.unwrap();
    if !unix::verify(password, &result.password_hash) {
        let authentication_error = AuthenticationError { error: Error::WrongPassword };
        socket_writer.write(&mut (&authentication_error).into());
        return None;
    }
    let user_id: u32 = result.id.try_into().unwrap();
    Some(user_id)
    // remove already logged user
}

pub fn list_characters(database: &Database, socket_writer: &SocketWriter, user_id: u32) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let rows = rt.block_on(async move {
        query!("SELECT * FROM players WHERE user_id = ? AND deleted_at IS NULL", user_id).fetch_all(&database.connection).await.unwrap()
    });
    let mut characters: Vec<PlayerCharacter> = vec![];
    for row in rows {
        let items_indexes: Vec<u16> = vec![];
        let character = PlayerCharacter { 
            id: row.id.try_into().unwrap(), 
            name: row.name.try_into().unwrap(), 
            class: row.class.try_into().unwrap(), 
            specialty: row.specialty.try_into().unwrap(), 
            level: row.level.try_into().unwrap(), 
            unknown1: vec![0, 0, 0, 0],
            base_strength: row.base_strength.try_into().unwrap(), 
            base_health: row.base_health.try_into().unwrap(), 
            base_intelligence: row.base_intelligence.try_into().unwrap(), 
            base_wisdom: row.base_wisdom.try_into().unwrap(), 
            base_agility: row.base_agility.try_into().unwrap(), 
            face: row.level.try_into().unwrap(), 
            hair: row.level.try_into().unwrap(),
            items_indexes 
        };
        characters.push(character);
    }
    let list_player_characters = ListPlayerCharacters { unknown1: vec![0, 0, 0, 0, 0], characters };
    socket_writer.write(&mut (&list_player_characters).into());
}

fn list_deleted_characters(database: &Database, socket_writer: &SocketWriter, user_id: u32) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let rows = rt.block_on(async move {
        query!("SELECT * FROM players WHERE user_id = ? AND deleted_at IS NOT NULL", user_id).fetch_all(&database.connection).await.unwrap()
    });
    let mut characters: Vec<PlayerDeletedCharacter> = vec![];
    for row in rows {
        let character = PlayerDeletedCharacter { 
            id: row.id.try_into().unwrap(), 
            name: row.name.try_into().unwrap(), 
            level: row.level.try_into().unwrap(), 
            remaining_days: 8,
            class: row.class.try_into().unwrap(), 
        };
        characters.push(character);
    }
    let list_player_characters = ListPlayerDeletedCharacters { characters };
    socket_writer.write(&mut (&list_player_characters).into());
}

fn delete_character(database: &Database, user_id: u32, character_id: u32) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        let now = Local::now().naive_utc();
        query!("UPDATE players SET deleted_at = ? WHERE user_id = ? AND id = ?", now, user_id, character_id).execute(&database.connection).await.unwrap()
    });
}

fn restore_deleted_character(database: &Database, user_id: u32, character_id: u32) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query!("UPDATE players SET deleted_at = NULL WHERE user_id = ? AND id = ?", user_id, character_id).execute(&database.connection).await.unwrap()
    });
}

fn is_name_taken(database: &Database, name: &str) -> bool {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query_scalar!("SELECT COUNT(*) FROM players WHERE name = ?", name).fetch_one(&database.connection).await.unwrap()
    }) > 0
}

fn get_characters_count(database: &Database, user_id: u32) -> i32 {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query_scalar!("SELECT COUNT(*) FROM players WHERE user_id = ? AND DELETED_AT is NULL", user_id).fetch_one(&database.connection).await.unwrap()
    })
}

fn create_character(database: &Database, socket_writer: &SocketWriter, user_id: u32, name: &str, base_strength: u8, base_health: u8, base_intelligence: u8, base_wisdom: u8, base_agility: u8, face: u8, hair: u8, class: PlayerClass) -> Option<u32> {
    if get_characters_count(database, user_id) >= 5 {
        return None;
    }
    if is_name_taken(database, name) {
        use crate::packets::server::character_creation_error::Error;
        let character_creation_error = CharacterCreationError { error: Error::NameTaken };
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

fn create_item(database: &Database, player_id: u32, index: u16, quantity: u32) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query!("
        INSERT INTO items 
        (player_id, item_index, prefix, quantity, maximum_endurance, current_endurance, physical_attack_talisman, magical_attack_talisman, talisman_of_accuracy, talisman_of_defence, upgrade_level, upgrade_rate) 
        values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ", player_id, index, 0, quantity, 0, 0, 0, 0, 0, 0, 0, 0)
        .execute(&database.connection).await.unwrap();
    });
}

fn handle_authentication(mut commands: Commands, query: Query<(Entity, &Authenticate, &SocketWriter)>, database: Res<Database>) {
    for (entity, client_packet, socket_writer) in &query {
        if let Some(user_id) = authenticate(&database, &socket_writer, &client_packet.username, &client_packet.password) {
            commands.entity(entity).insert(User { id: user_id });    
            list_characters(&database, &socket_writer, user_id);
            list_deleted_characters(&database, &socket_writer, user_id);
        }
        commands.entity(entity).remove::<Authenticate>();
    }
}

fn handle_delete_character(mut commands: Commands, query: Query<(Entity, &User, &DeleteCharacter, &SocketWriter)>, database: Res<Database>) {
    for (entity, user, client_packet, socket_writer) in &query {
        delete_character(&database, user.id, client_packet.character_id);
        list_characters(&database, &socket_writer, user.id);
        list_deleted_characters(&database, &socket_writer, user.id);
        commands.entity(entity).remove::<DeleteCharacter>();
    }
}

fn handle_restore_deleted_character(mut commands: Commands, query: Query<(Entity, &User, &RestoreDeletedCharacter, &SocketWriter)>, database: Res<Database>) {
    for (entity, user, client_packet, socket_writer) in &query {
        restore_deleted_character(&database, user.id, client_packet.character_id);
        list_characters(&database, &socket_writer, user.id);
        list_deleted_characters(&database, &socket_writer, user.id);
        commands.entity(entity).remove::<RestoreDeletedCharacter>();
    }
}

fn handle_create_character(mut commands: Commands, query: Query<(Entity, &User, &CreateCharacter, &SocketWriter)>, database: Res<Database>) {
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
            create_item(&database, player_id, 1, 100);
            create_item(&database, player_id, 47, 100);
        }
        list_characters(&database, &socket_writer, user.id);
        list_deleted_characters(&database, &socket_writer, user.id);
        commands.entity(entity).remove::<CreateCharacter>();
    }
}