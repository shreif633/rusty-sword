use bevy::prelude::*;
use crate::components::aggro::Aggro;
use crate::components::dead::Dead;
use crate::components::experience::Experience;
use crate::components::experience_rate::ExperienceRate;
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
        app.add_systems(Update, attempt_level_up);
        app.add_systems(Update, add_level_up_effect);
        app.add_systems(Update, broadcast_new_level);
    }
}

fn distribute_experience(mut query: Query<(&mut Aggro, &Experience, &Level), (Added<Dead>, Without<Player>)>, mut players: Query<(&mut Experience, &ExperienceRate, &Level, &SocketWriter), With<Player>>) {
    for (mut aggro, target_experience, target_level) in query.iter_mut() {
        let total_aggro: u32 = aggro.list.values().sum();
        for (entity, points) in &aggro.list {
            if let Ok((mut experience, experience_rate, player_level, socket_writer)) = players.get_mut(*entity) {
                let percentage: i64 = (total_aggro * 100 / points).into();
                println!("percentage {}", percentage);
                let partial_experience = target_experience.experience * 100 / percentage;
                println!("partial_experience {}", partial_experience);
                let partial_experience = partial_experience * experience_rate.percentage as i64 / 100;
                println!("partial_experience rate {}", partial_experience);
                let partial_experience = partial_experience * player_level.get_target_color(target_level.level).experience_rate() as i64 / 100;
                println!("partial_experience color {}", partial_experience);
                experience.experience += partial_experience;
                let player_experience_response = PlayerExperienceResponse { current_experience: experience.experience, added_experience: partial_experience };
                socket_writer.write(&mut (&player_experience_response).into());
            }
        }
        aggro.list.clear();
    }
}

fn attempt_level_up(mut query: Query<(&mut Level, &Experience), Changed<Experience>>) {
    for (mut level, experience) in query.iter_mut() {
        while experience.should_level_up(level.level) {
            level.level += 1;
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