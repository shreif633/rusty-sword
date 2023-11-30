use bevy::prelude::*;
use sqlx::{types::chrono::NaiveDateTime, query};
use crate::components::appearence::Appearence;
use crate::components::base_points::BasePoints;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::equipped_weapon::EquippedWeapon;
use crate::components::experience::Experience;
use crate::components::extra_points::ExtraPoints;
use crate::components::final_points::FinalPoints;
use crate::components::job::Job;
use crate::components::magical_attack::MagicalAttack;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::maximum_magic_points::MaximumMagicPoints;
use crate::components::physical_attack::PhysicalAttack;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::rage::Rage;
use crate::components::user::User;
use crate::framework::database::Database;
use crate::responses::player_extra_health::PlayerExtraHealthResponse;
use crate::responses::player_extra_strength::PlayerExtraStrengthResponse;
use crate::responses::player_extra_intelligence::PlayerExtraIntelligenceResponse;
use crate::responses::player_extra_wisdom::PlayerExtraWisdomResponse;
use crate::responses::player_extra_agility::PlayerExtraAgilityResponse;
use crate::responses::player_information::PlayerInformationResponse;
use crate::requests::select_character::SelectCharacterRequest;
use super::inventory::OldWeapon;
use super::tcp_server::SocketWriter;
use super::player_movement::PreviousPosition;

pub struct SelectCharacterPlugin;

impl Plugin for SelectCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_select_character);
        app.add_systems(Update, character_information);
    }
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
    extra_strength: u16, 
    extra_health: u16, 
    extra_intelligence: u16, 
    extra_wisdom: u16,
    extra_agility: u16,  
    minimum_physical_attack: u16,
    maximum_physical_attack: u16,
    minimum_magical_attack: u16,
    maximum_magical_attack: u16,
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
    rage: u32,
    on_target_point: u16, 
    evasion: u16, 
    defense: u16, 
    absorption: u16, 
    fire_resistence: u16, 
    ice_resistence: u16, 
    lighning_resistence: u16,
    curse_resistence: u16, 
    non_elemental_resistence: u16,
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

fn handle_select_character(mut commands: Commands, query: Query<(Entity, &User, &SelectCharacterRequest)>, database: Res<Database>) {
    for (entity, user, client_packet) in &query {
        if let Some(player_row) = query_player(&database, user.id, client_packet.character_id) {
            commands.entity(entity).insert(Player { 
                id: player_row.id, 
            });
            commands.entity(entity).insert(Job { 
                class: player_row.class,
                level: player_row.level,
                specialty: player_row.specialty,  
            });
            commands.entity(entity).insert(BasePoints {
                base_strength: player_row.base_strength,
                base_health: player_row.base_health,
                base_intelligence: player_row.base_intelligence,
                base_wisdom: player_row.base_wisdom,
                base_agility: player_row.base_agility,
            });
            commands.entity(entity).insert(ExtraPoints {
                extra_strength: player_row.extra_strength,
                extra_health: player_row.extra_health,
                extra_intelligence: player_row.extra_intelligence,
                extra_wisdom: player_row.extra_wisdom,
                extra_agility: player_row.extra_agility,
            });
            commands.entity(entity).insert(FinalPoints {
                on_target_point: player_row.on_target_point,
                evasion: player_row.evasion,
                defense: player_row.defense,
                absorption: player_row.absorption,
                fire_resistence: player_row.fire_resistence,
                ice_resistence: player_row.ice_resistence,
                lighning_resistence: player_row.lighning_resistence,
                curse_resistence: player_row.curse_resistence,
                non_elemental_resistence: player_row.non_elemental_resistence,
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
            commands.entity(entity).insert(MaximumHealthPoints {
                maximum_health_points: player_row.maximum_health_points,
            });
            commands.entity(entity).insert(CurrentHealthPoints {
                current_health_points: player_row.current_health_points,
            });
            commands.entity(entity).insert(MaximumMagicPoints {
                maximum_magic_points: player_row.maximum_magic_points,
            });
            commands.entity(entity).insert(CurrentMagicPoints {
                current_magic_points: player_row.current_magic_points,
            });
            commands.entity(entity).insert(Experience { 
                experience: player_row.experience
            });
            commands.entity(entity).insert(Rage { 
                rage: player_row.rage
            });
            commands.entity(entity).insert(PhysicalAttack {
                minimum_physical_attack: player_row.minimum_physical_attack,
                maximum_physical_attack: player_row.maximum_physical_attack,
            });
            commands.entity(entity).insert(MagicalAttack {
                minimum_magical_attack: player_row.minimum_magical_attack,
                maximum_magical_attack: player_row.minimum_magical_attack,
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
            commands.entity(entity).insert(EquippedWeapon {
                item: None 
            });
            commands.entity(entity).insert(OldWeapon {
                item: None 
            });
        }
        commands.entity(entity).remove::<SelectCharacterRequest>();
    }
}

fn character_information(query: Query<(Added<Player>, &Job, &BasePoints, &ExtraPoints, &FinalPoints, &PhysicalAttack, &MagicalAttack, &CurrentHealthPoints, &MaximumHealthPoints, &CurrentMagicPoints, &MaximumMagicPoints, &Rage, &Experience, &SocketWriter)>) {
    for (player_added, job, base_points, extra_points, final_points, physical_attack, magical_attack, current_health_points, maximum_health_points, current_magic_points, maximum_magic_points, rage, experience, socket_writer) in &query {
        if player_added {
            let player_information = PlayerInformationResponse { 
                specialization: job.specialty, 
                unknown1: vec![0, 0], 
                contribution: 9, 
                base_strength: base_points.base_strength, 
                base_health: base_points.base_health, 
                base_intelligence: base_points.base_intelligence, 
                base_wisdom: base_points.base_wisdom, 
                base_agility: base_points.base_agility, 
                current_health_points: current_health_points.current_health_points, 
                maximum_health_points: maximum_health_points.maximum_health_points, 
                current_magic_points: current_magic_points.current_magic_points, 
                maximum_magic_points: maximum_magic_points.maximum_magic_points, 
                on_target_point: final_points.on_target_point, 
                evasion: final_points.evasion, 
                defense: final_points.defense, 
                absorption: final_points.absorption, 
                experience: experience.experience,
                unknown2: vec![0, 0, 0], 
                minimum_physical_attack: physical_attack.minimum_physical_attack, 
                maximum_physical_attack: physical_attack.maximum_physical_attack, 
                minimum_magical_attack: magical_attack.minimum_magical_attack, 
                maximum_magical_attack: magical_attack.maximum_magical_attack, 
                status_points: 20, 
                skill_points: 44, 
                fire_resistence: final_points.fire_resistence.try_into().unwrap(), 
                ice_resistence: final_points.ice_resistence.try_into().unwrap(), 
                lighning_resistence: final_points.lighning_resistence.try_into().unwrap(), 
                curse_resistence: final_points.curse_resistence.try_into().unwrap(), 
                non_elemental_resistence: final_points.non_elemental_resistence.try_into().unwrap(), 
                rage: rage.rage 
            };
            socket_writer.write(&mut (&player_information).into());

            let player_extra_health = PlayerExtraHealthResponse { 
                extra_health: extra_points.extra_health, 
                current_health_points: current_health_points.current_health_points, 
                maximum_health_points: maximum_health_points.maximum_health_points, 
                non_elemental_resistence: final_points.non_elemental_resistence 
            };
            socket_writer.write(&mut (&player_extra_health).into());

            let player_extra_strength = PlayerExtraStrengthResponse { 
                extra_strength: extra_points.extra_strength, 
                on_target_point: final_points.on_target_point, 
                minimum_physical_attack: physical_attack.minimum_physical_attack, 
                maximum_physical_attack: physical_attack.maximum_physical_attack 
            };
            socket_writer.write(&mut (&player_extra_strength).into());

            let player_extra_intelligence = PlayerExtraIntelligenceResponse { 
                extra_intelligence: extra_points.extra_intelligence, 
                minimum_magical_attack: magical_attack.minimum_magical_attack, 
                maximum_magical_attack: magical_attack.maximum_magical_attack, 
                fire_resistence: final_points.fire_resistence, 
                ice_resistence: final_points.ice_resistence, 
                lighning_resistence: final_points.lighning_resistence 
            };
            socket_writer.write(&mut (&player_extra_intelligence).into());

            let player_extra_wisdom = PlayerExtraWisdomResponse { 
                extra_wisdom: extra_points.extra_wisdom, 
                current_magic_points: current_magic_points.current_magic_points, 
                maximum_magic_points: maximum_magic_points.maximum_magic_points, 
                minimum_magical_attack: magical_attack.minimum_magical_attack, 
                maximum_magical_attack: magical_attack.maximum_magical_attack, 
                curse_resistence: final_points.curse_resistence 
            };
            socket_writer.write(&mut (&player_extra_wisdom).into());

            let player_extra_agility = PlayerExtraAgilityResponse {
                extra_agility: extra_points.extra_agility, 
                on_target_point: final_points.on_target_point, 
                evasion: final_points.evasion, 
                unknown_evasion_copy: final_points.evasion, 
                minimum_physical_attack: physical_attack.minimum_physical_attack, 
                maximum_physical_attack: physical_attack.maximum_physical_attack
            };
            socket_writer.write(&mut (&player_extra_agility).into());
        }
    }
}