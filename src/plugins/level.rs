use bevy::prelude::*;
use crate::components::aggro::Aggro;
use crate::components::dead::Dead;
use crate::components::experience::Experience;
use crate::components::level::Level;
use crate::components::player::Player;
use crate::components::visual_effect::VisualEffect;
use crate::responses::player_experience::PlayerExperienceResponse;
use crate::responses::player_level::PlayerLevelResponse;
use super::tcp_server::SocketWriter;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, distribute_experience);
        app.add_systems(Update, compare_level);
        app.add_systems(Update, add_level_up_effect);
        app.add_systems(Update, broadcast_new_level);
    }
}

fn distribute_experience(mut query: Query<(&mut Aggro, &Experience), (Added<Dead>, Without<Player>)>, mut players: Query<(&mut Experience, &SocketWriter), With<Player>>) {
    for (mut aggro, experience) in query.iter_mut() {
        let total_aggro: u32 = aggro.list.values().sum();
        for (entity, points) in &aggro.list {
            let percentage: i64 = (total_aggro * 100 / points).into();
            let partial_experience = experience.experience * 100 / percentage;
            if let Ok((mut experience, socket_writer)) = players.get_mut(*entity) {
                experience.experience += partial_experience;
                let player_experience_response = PlayerExperienceResponse { current_experience: experience.experience, added_experience: partial_experience };
                socket_writer.write(&mut (&player_experience_response).into());
            }
        }
        aggro.list.clear();
    }
}

fn compare_level(mut query: Query<(&mut Level, &Experience), Changed<Experience>>) {
    for (mut level, experience) in query.iter_mut() {
        let calculated_level = experience.calculate_level();
        if calculated_level != level.level {
            level.level = calculated_level;
        }
    }
}

fn broadcast_new_level(query: Query<(&Level, &SocketWriter), Changed<Level>>) {
    for (level, socket_writer) in &query {
        let player_level_response = PlayerLevelResponse { level: level.level };
        socket_writer.write(&mut (&player_level_response).into());
    }
}

fn add_level_up_effect(mut commands: Commands, query: Query<Entity, Changed<Level>>) {
    for entity in &query {
        commands.entity(entity).insert(VisualEffect { visual_effect: "effect_levelup".to_string() });
    }
}