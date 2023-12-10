use bevy::prelude::*;
use crate::bundles::player::PlayerBundle;
use crate::components::admin::Admin;
use crate::components::base_points::BasePoints;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::experience::Experience;
use crate::components::extra_points::ExtraPoints;
use crate::components::final_points::FinalPoints;
use crate::components::magical_attack::MagicalAttack;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::maximum_magic_points::MaximumMagicPoints;
use crate::components::physical_attack::PhysicalAttack;
use crate::components::player::Player;
use crate::components::rage::Rage;
use crate::components::user::User;
use crate::framework::database::Database;
use crate::framework::entity_map::EntityMap;
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

fn handle_select_character(mut commands: Commands, query: Query<(Entity, &User, &SelectCharacterRequest)>, database: Res<Database>, mut players_map: ResMut<EntityMap<Player>>) {
    for (entity, user, client_packet) in &query {
        if let Some(player_row) = find_user_player_by_id(&database, user.id, client_packet.character_id) {
            players_map.map.insert(player_row.id, entity);
            commands.entity(entity).insert(PlayerBundle::new(&player_row));
            commands.entity(entity).insert(Admin);
        }
        commands.entity(entity).remove::<SelectCharacterRequest>();
    }
}

fn character_information(query: Query<(&Player, &BasePoints, &ExtraPoints, &FinalPoints, &PhysicalAttack, &MagicalAttack, &CurrentHealthPoints, &MaximumHealthPoints, &CurrentMagicPoints, &MaximumMagicPoints, &Rage, &Experience, &SocketWriter), Added<Player>>) {
    for (player, base_points, extra_points, final_points, physical_attack, magical_attack, current_health_points, maximum_health_points, current_magic_points, maximum_magic_points, rage, experience, socket_writer) in &query {
        let player_information = PlayerInformationResponse::new(player, base_points, current_health_points, maximum_health_points, current_magic_points, maximum_magic_points, final_points, experience, physical_attack, magical_attack, rage);
        socket_writer.write(&mut (&player_information).into());
        let player_extra_health = PlayerExtraHealthResponse::new(extra_points, current_health_points, maximum_health_points, final_points);
        socket_writer.write(&mut (&player_extra_health).into());
        let player_extra_strength = PlayerExtraStrengthResponse::new(extra_points, final_points, physical_attack);
        socket_writer.write(&mut (&player_extra_strength).into());
        let player_extra_intelligence = PlayerExtraIntelligenceResponse::new(extra_points, magical_attack, final_points);
        socket_writer.write(&mut (&player_extra_intelligence).into());
        let player_extra_wisdom = PlayerExtraWisdomResponse::new(extra_points, current_magic_points, maximum_magic_points, magical_attack, final_points);
        socket_writer.write(&mut (&player_extra_wisdom).into());
        let player_extra_agility = PlayerExtraAgilityResponse::new(extra_points, final_points, physical_attack);
        socket_writer.write(&mut (&player_extra_agility).into());
    }
}