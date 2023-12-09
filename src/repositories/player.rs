use sqlx::{query, query_scalar};
use sqlx::types::chrono::{NaiveDateTime, Local};
use crate::enums::player_class::PlayerClass;
use crate::framework::database::Database;

pub struct PlayerRow {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub class: u8,
    pub level: u8,
    pub specialty: u8,
    pub base_strength: u16, 
    pub base_health: u16, 
    pub base_intelligence: u16, 
    pub base_wisdom: u16,
    pub base_agility: u16,  
    pub extra_strength: u16, 
    pub extra_health: u16, 
    pub extra_intelligence: u16, 
    pub extra_wisdom: u16,
    pub extra_agility: u16,  
    pub minimum_physical_attack: u16,
    pub maximum_physical_attack: u16,
    pub minimum_magical_attack: u16,
    pub maximum_magical_attack: u16,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub face: u8,
    pub hair: u8,
    pub weapon_index: u16, 
    pub shield_index: u16, 
    pub helmet_index: u16, 
    pub chest_index: u16, 
    pub shorts_index: u16, 
    pub gloves_index: u16, 
    pub boots_index: u16, 
    pub current_health_points: u32,
    pub maximum_health_points: u32,
    pub current_magic_points: u16,
    pub maximum_magic_points: u16,
    pub experience: i64,
    pub deleted_at: Option<NaiveDateTime>, 
    pub rage: u32,
    pub on_target_point: u16, 
    pub evasion: u16, 
    pub defense: u16, 
    pub absorption: u8, 
    pub fire_resistence: u16, 
    pub ice_resistence: u16, 
    pub lighning_resistence: u16,
    pub curse_resistence: u16, 
    pub non_elemental_resistence: u16,
}

pub fn find_user_player_by_id(database: &Database, user_id: i32, player_id: i32) -> Option<PlayerRow> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let row = rt.block_on(async move {
        let result = query!("SELECT * FROM players WHERE id = ? AND user_id = ? AND deleted_at IS NULL", player_id, user_id).fetch_one(&database.connection).await;
        match result {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    });
    match row {
        Some(row) => Some(PlayerRow { 
            id: player_id, 
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
            experience: row.experience, 
            deleted_at: row.deleted_at, 
            rage: row.rage.try_into().unwrap(),
            extra_strength: row.extra_strength.try_into().unwrap(),
            extra_health: row.extra_health.try_into().unwrap(), 
            extra_intelligence: row.extra_intelligence.try_into().unwrap(),
            extra_wisdom: row.extra_wisdom.try_into().unwrap(),
            extra_agility: row.extra_agility.try_into().unwrap(), 
            minimum_physical_attack: row.minimum_physical_attack.try_into().unwrap(), 
            maximum_physical_attack: row.maximum_physical_attack.try_into().unwrap(), 
            minimum_magical_attack: row.minimum_magical_attack.try_into().unwrap(), 
            maximum_magical_attack: row.maximum_magical_attack.try_into().unwrap(), 
            on_target_point: row.on_target_point.try_into().unwrap(), 
            evasion: row.evasion.try_into().unwrap(), 
            defense: row.defense.try_into().unwrap(),  
            absorption: row.absorption.try_into().unwrap(), 
            fire_resistence: row.fire_resistence.try_into().unwrap(), 
            ice_resistence: row.ice_resistence.try_into().unwrap(), 
            lighning_resistence: row.lighning_resistence.try_into().unwrap(), 
            curse_resistence: row.curse_resistence.try_into().unwrap(), 
            non_elemental_resistence: row.non_elemental_resistence.try_into().unwrap(), 
        }),
        None => None,
    }
}

pub fn delete_user_player_by_id(database: &Database, user_id: i32, character_id: i32) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        let now = Local::now().naive_utc();
        query!("UPDATE players SET deleted_at = ? WHERE user_id = ? AND id = ?", now, user_id, character_id).execute(&database.connection).await.unwrap()
    });
}

pub fn restore_user_player_by_id(database: &Database, user_id: i32, character_id: i32) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query!("UPDATE players SET deleted_at = NULL WHERE user_id = ? AND id = ?", user_id, character_id).execute(&database.connection).await.unwrap()
    });
}

pub fn find_player_exists_by_name(database: &Database, name: &str) -> bool {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query_scalar!("SELECT COUNT(*) FROM players WHERE name = ?", name).fetch_one(&database.connection).await.unwrap()
    }) > 0
}

pub fn count_user_players(database: &Database, user_id: i32) -> i32 {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query_scalar!("SELECT COUNT(*) FROM players WHERE user_id = ? AND DELETED_AT IS NULL", user_id).fetch_one(&database.connection).await.unwrap()
    })
}

pub fn find_all_deleted_user_players(database: &Database, user_id: i32) -> Vec<PlayerRow> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let rows = rt.block_on(async move {
        query!("SELECT * FROM players WHERE user_id = ? AND deleted_at IS NOT NULL", user_id).fetch_all(&database.connection).await.unwrap()
    });
    rows.iter().map(|row| {
        PlayerRow { 
            id: row.id.try_into().unwrap(), 
            user_id, 
            name: row.name.clone(), 
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
            experience: row.experience, 
            deleted_at: row.deleted_at, 
            rage: row.rage.try_into().unwrap(),
            extra_strength: row.extra_strength.try_into().unwrap(),
            extra_health: row.extra_health.try_into().unwrap(), 
            extra_intelligence: row.extra_intelligence.try_into().unwrap(),
            extra_wisdom: row.extra_wisdom.try_into().unwrap(),
            extra_agility: row.extra_agility.try_into().unwrap(), 
            minimum_physical_attack: row.minimum_physical_attack.try_into().unwrap(), 
            maximum_physical_attack: row.maximum_physical_attack.try_into().unwrap(), 
            minimum_magical_attack: row.minimum_magical_attack.try_into().unwrap(), 
            maximum_magical_attack: row.maximum_magical_attack.try_into().unwrap(), 
            on_target_point: row.on_target_point.try_into().unwrap(), 
            evasion: row.evasion.try_into().unwrap(), 
            defense: row.defense.try_into().unwrap(),  
            absorption: row.absorption.try_into().unwrap(), 
            fire_resistence: row.fire_resistence.try_into().unwrap(), 
            ice_resistence: row.ice_resistence.try_into().unwrap(), 
            lighning_resistence: row.lighning_resistence.try_into().unwrap(), 
            curse_resistence: row.curse_resistence.try_into().unwrap(), 
            non_elemental_resistence: row.non_elemental_resistence.try_into().unwrap(), 
        }
    }).collect()
}

pub fn find_all_user_players(database: &Database, user_id: i32) -> Vec<PlayerRow> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let rows = rt.block_on(async move {
        query!("SELECT * FROM players WHERE user_id = ? AND deleted_at IS NULL", user_id).fetch_all(&database.connection).await.unwrap()
    });
    rows.iter().map(|row| {
        PlayerRow { 
            id: row.id.try_into().unwrap(), 
            user_id, 
            name: row.name.clone(), 
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
            experience: row.experience, 
            deleted_at: row.deleted_at, 
            rage: row.rage.try_into().unwrap(),
            extra_strength: row.extra_strength.try_into().unwrap(),
            extra_health: row.extra_health.try_into().unwrap(), 
            extra_intelligence: row.extra_intelligence.try_into().unwrap(),
            extra_wisdom: row.extra_wisdom.try_into().unwrap(),
            extra_agility: row.extra_agility.try_into().unwrap(), 
            minimum_physical_attack: row.minimum_physical_attack.try_into().unwrap(), 
            maximum_physical_attack: row.maximum_physical_attack.try_into().unwrap(), 
            minimum_magical_attack: row.minimum_magical_attack.try_into().unwrap(), 
            maximum_magical_attack: row.maximum_magical_attack.try_into().unwrap(), 
            on_target_point: row.on_target_point.try_into().unwrap(), 
            evasion: row.evasion.try_into().unwrap(), 
            defense: row.defense.try_into().unwrap(),  
            absorption: row.absorption.try_into().unwrap(), 
            fire_resistence: row.fire_resistence.try_into().unwrap(), 
            ice_resistence: row.ice_resistence.try_into().unwrap(), 
            lighning_resistence: row.lighning_resistence.try_into().unwrap(), 
            curse_resistence: row.curse_resistence.try_into().unwrap(), 
            non_elemental_resistence: row.non_elemental_resistence.try_into().unwrap(), 
        }
    }).collect()
}

pub struct PlayerCreateChangeset<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub class: PlayerClass,
    pub experience: u32,
    pub base_strength: u16,
    pub base_health: u16,
    pub base_intelligence: u16,
    pub base_wisdom: u16,
    pub base_agility: u16,
    pub face: u8,
    pub hair: u8,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub map: u8
}

pub fn create_player(database: &Database, changeset: &PlayerCreateChangeset) -> Option<i32> {
    let class = u8::from(changeset.class);
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let level = changeset.experience + 1;
    let player_id = rt.block_on(async move {
        query_scalar!("
        INSERT INTO players 
        (user_id, name, class, specialty, level, base_strength, base_health, base_intelligence, base_wisdom, base_agility, face, hair, x, y, z, weapon_index, shield_index, helmet_index, chest_index, shorts_index, gloves_index, boots_index, current_health_points, maximum_health_points, current_magic_points, maximum_magic_points, experience, rage) 
        values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        ", changeset.user_id, changeset.name, class, 1, level, changeset.base_strength, changeset.base_health, changeset.base_intelligence, changeset.base_wisdom, changeset.base_agility, changeset.face, changeset.hair, changeset.x, changeset.y, changeset.z, 0, 0, 0, 0, 0, 0, 0, 1000, 2000, 1000, 2000, 0, 0)
        .fetch_one(&database.connection).await.unwrap()
    }) as i32;
    Some(player_id)
}

pub struct PlayerUpdatePositionChangeset {
    pub id: i32,
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub fn update_all_player_position_by_id(
    database: &Database,
    changesets: &Vec<PlayerUpdatePositionChangeset>
) {
    let mut update = "".to_string();
    for changeset in changesets {
        update.push_str(&format!("UPDATE players SET x = {}, y = {}, z = {} WHERE id = {};", changeset.x, changeset.y, changeset.z, changeset.id));
    }
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query(&update).execute(&database.connection).await.unwrap()
    });
} 