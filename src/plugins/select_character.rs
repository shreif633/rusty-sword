use bevy::prelude::*;
use sqlx::{types::chrono::NaiveDateTime, query};
use crate::{packets::client::select_character::SelectCharacter, framework::database::Database};
use super::{character_selection::User, player_movement::{Position, PreviousPosition}};

pub struct SelectCharacterPlugin;

impl Plugin for SelectCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_select_character);
    }
}

#[derive(Component)]
pub struct Player {
    pub id: u32,
    pub class: u8,
}

#[derive(Component)]
pub struct Appearence {
    pub name: String,
    pub face: u8,
    pub hair: u8,
    pub weapon_index: u16, 
    pub shield_index: u16, 
    pub helmet_index: u16, 
    pub chest_index: u16, 
    pub shorts_index: u16, 
    pub gloves_index: u16, 
    pub boots_index: u16, 
}

#[derive(Component)]
struct PlayerBuild {
    pub level: u8,
    pub specialty: u8,
    pub base_strength: u16, 
    pub base_health: u16, 
    pub base_intelligence: u16, 
    pub base_wisdom: u16,
    pub base_agility: u16,  
}

struct PlayerRow {
    id: u32,
    user_id: u32,
    name: String,
    class: u8,
    level: u8,
    specialty: u8,
    base_strength: u16, 
    base_health: u16, 
    base_intelligence: u16, 
    base_wisdom: u16,
    base_agility: u16,  
    x: u32,
    y: u32,
    z: u32,
    face: u8,
    hair: u8,
    weapon_index: u16, 
    shield_index: u16, 
    helmet_index: u16, 
    chest_index: u16, 
    shorts_index: u16, 
    gloves_index: u16, 
    boots_index: u16, 
    current_health_points: u32,
    maximum_health_points: u32,
    current_magic_points: u16,
    maximum_magic_points: u16,
    experience: u32,
    deleted_at: Option<NaiveDateTime>, 
    rage: u32
}

fn query_player(database: &Database, user_id: u32, character_id: u32) -> Option<PlayerRow> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let row = rt.block_on(async move {
        let result = query!("SELECT * FROM players WHERE id = ? AND user_id = ? AND deleted_at IS NULL", character_id, user_id).fetch_one(&database.connection).await;
        match result {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    });
    match row {
        Some(row) => Some(PlayerRow { 
            id: character_id, 
            user_id, 
            name: row.name, 
            class: row.class.try_into().unwrap(), 
            level: row.level.try_into().unwrap(), 
            specialty: row.specialty.try_into().unwrap(), 
            base_strength: row.base_strength.try_into().unwrap(), 
            base_health: row.base_health.try_into().unwrap(), 
            base_intelligence: row.base_intelligence.try_into().unwrap(), 
            base_wisdom: row.base_wisdom.try_into().unwrap(), 
            base_agility: row.base_agility.try_into().unwrap(), 
            x: row.x.try_into().unwrap(), 
            y: row.y.try_into().unwrap(), 
            z: row.z.try_into().unwrap(), 
            face: row.face.try_into().unwrap(), 
            hair: row.hair.try_into().unwrap(), 
            weapon_index: row.weapon_index.try_into().unwrap(), 
            shield_index: row.shield_index.try_into().unwrap(), 
            helmet_index: row.helmet_index.try_into().unwrap(), 
            chest_index: row.chest_index.try_into().unwrap(), 
            shorts_index: row.shorts_index.try_into().unwrap(), 
            gloves_index: row.gloves_index.try_into().unwrap(), 
            boots_index: row.boots_index.try_into().unwrap(), 
            current_health_points: row.current_health_points.try_into().unwrap(), 
            maximum_health_points: row.maximum_health_points.try_into().unwrap(), 
            current_magic_points: row.current_magic_points.try_into().unwrap(), 
            maximum_magic_points: row.maximum_magic_points.try_into().unwrap(), 
            experience: row.experience.try_into().unwrap(), 
            deleted_at: row.deleted_at, 
            rage: row.rage.try_into().unwrap() 
        }),
        None => None,
    }
    
}

fn handle_select_character(mut commands: Commands, query: Query<(Entity, &User, &SelectCharacter)>, database: Res<Database>) {
    for (entity, user, client_packet) in &query {
        if let Some(player_row) = query_player(&database, user.id, client_packet.character_id) {
            commands.entity(entity).insert(Player { 
                id: player_row.id, 
                class: player_row.class,  
            });
            commands.entity(entity).insert(PlayerBuild {
                level: player_row.level,
                specialty: player_row.specialty,
                base_strength: player_row.base_strength,
                base_health: player_row.base_health,
                base_intelligence: player_row.base_intelligence,
                base_wisdom: player_row.base_wisdom,
                base_agility: player_row.base_agility,
            });
            commands.entity(entity).insert(PreviousPosition { 
                x: 0, 
                y: 0, 
                z: 0
            });
            commands.entity(entity).insert(Position { 
                x: player_row.x, 
                y: player_row.y, 
                z: player_row.z
            });
            commands.entity(entity).insert(Appearence {
                name: player_row.name,
                face: player_row.face,
                hair: player_row.hair,
                weapon_index: player_row.weapon_index,
                shield_index: player_row.shield_index,
                helmet_index: player_row.helmet_index,
                chest_index: player_row.chest_index,
                shorts_index: player_row.shorts_index,
                gloves_index: player_row.gloves_index,
                boots_index: player_row.boots_index,
            });
        }
        commands.entity(entity).remove::<SelectCharacter>();
    }
}