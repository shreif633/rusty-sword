use bevy::prelude::*;
use crate::bundles::player::PlayerBundle;
use crate::components::base_points::BasePoints;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::experience::Experience;
use crate::components::extra_points::ExtraPoints;
use crate::components::final_points::FinalPoints;
use crate::components::job::Job;
use crate::components::magical_attack::MagicalAttack;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::maximum_magic_points::MaximumMagicPoints;
use crate::components::physical_attack::PhysicalAttack;
use crate::components::player::Player;
use crate::components::rage::Rage;
use crate::components::user::User;
use crate::framework::database::Database;
use crate::repositories::player::find_user_player_by_id;
use crate::responses::player_extra_health::PlayerExtraHealthResponse;
use crate::responses::player_extra_strength::PlayerExtraStrengthResponse;
use crate::responses::player_extra_intelligence::PlayerExtraIntelligenceResponse;
use crate::responses::player_extra_wisdom::PlayerExtraWisdomResponse;
use crate::responses::player_extra_agility::PlayerExtraAgilityResponse;
use crate::responses::player_information::PlayerInformationResponse;
use crate::requests::select_character::SelectCharacterRequest;
use super::tcp_server::SocketWriter;

pub struct SelectCharacterPlugin;

impl Plugin for SelectCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_select_character);
        app.add_systems(Update, character_information);
    }
}

fn handle_select_character(mut commands: Commands, query: Query<(Entity, &User, &SelectCharacterRequest)>, database: Res<Database>) {
    for (entity, user, client_packet) in &query {
        if let Some(player_row) = find_user_player_by_id(&database, user.id, client_packet.character_id) {
            commands.entity(entity).insert(PlayerBundle::from(&player_row));
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