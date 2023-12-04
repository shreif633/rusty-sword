use bevy::prelude::*;
use crate::configs::monsters::MonstersConfig;
use crate::bundles::monster::MonsterBundle;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::responses::monster_appear::MonsterAppearResponse;
use crate::components::monster::Monster;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::maximum_health_points::MaximumHealthPoints;
use rand::{thread_rng, Rng};
use super::tcp_server::SocketWriter;

pub struct SpawnMonstersPlugin;

impl Plugin for SpawnMonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_monsters);
        app.add_systems(Update, handle_position_change);
    }
}

fn spawn_monsters(mut commands: Commands, monsters_configs: Res<MonstersConfig>) {
    let mut rng = thread_rng();
    for (monster_index, monster_config) in monsters_configs.config.iter() {
        for spawn in &monster_config.spawn {
            for _ in 0..spawn.quantity {
                let random_x = rng.gen_range(spawn.bottom_x..spawn.top_x);
                let random_y = rng.gen_range(spawn.bottom_y..spawn.top_y);
                commands.spawn(
                    MonsterBundle { 
                        monster: Monster { index: *monster_index }, 
                        previous_position: Previous::from(Position { x: random_x, y: random_y, z: 0 }), 
                        position: Position { x: random_x, y: random_y, z: 0 }, 
                        maximum_health_points: MaximumHealthPoints { maximum_health_points: 100 }, 
                        current_health_points: CurrentHealthPoints { current_health_points: 100 }, 
                        previous_current_health_points: Previous::from(CurrentHealthPoints { current_health_points: 100 }),
                    }
                );
            }
        }
    }
}

fn handle_position_change(moved_query: Query<(&Previous<Position>, &Position, &SocketWriter), Changed<Position>>, monsters_query: Query<(Entity, &Monster, &Position, &CurrentHealthPoints, &MaximumHealthPoints)>) {
    for (moved_previous_position, moved_position, moved_socket_writer) in &moved_query {
        for (entity, monster, position, current_health_points, maximum_health_points) in &monsters_query {
            if !position.is_in_sight(&moved_previous_position.entity) && position.is_in_sight(moved_position) {
                let player_appear = MonsterAppearResponse::new(entity, monster, position, current_health_points, maximum_health_points);
                moved_socket_writer.write(&mut (&player_appear).into());
            }
        }
    }
}