use bevy::prelude::*;
use crate::components::beheadable::Beheadable;
use crate::components::spawn::Spawn;
use crate::framework::entity_map::EntityMap;
use crate::{configs::monsters::MonstersConfig, components::id::Id};
use crate::bundles::monster::MonsterBundle;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::responses::monster_appear::MonsterAppearResponse;
use crate::components::monster::Monster;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::maximum_health_points::MaximumHealthPoints;
use super::tcp_server::SocketWriter;

pub struct SpawnMonstersPlugin;

impl Plugin for SpawnMonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_monsters);
        app.add_systems(PostUpdate, handle_position_change);
        app.add_systems(Last, handle_position_added);
    }
}

fn spawn_monsters(mut commands: Commands, monsters_configs: Res<MonstersConfig>, mut monsters_map: ResMut<EntityMap<Monster>>) {
    let mut monster_id: i32 = 0;
    for (monster_index, monster_config) in monsters_configs.config.iter() {
        for spawn_config in &monster_config.spawn {
            let spawn = Spawn { bottom_x: spawn_config.bottom_x, bottom_y: spawn_config.bottom_y, top_x: spawn_config.top_x, top_y: spawn_config.top_y };
            for _ in 0..spawn_config.quantity {
                let position = Position::from(&spawn);
                monster_id -= 1;
                let entity = commands.spawn(
                    MonsterBundle {
                        id: Id { id: monster_id },
                        monster: Monster { index: *monster_index }, 
                        previous_position: Previous::from(position.clone()), 
                        position: position.clone(), 
                        maximum_health_points: MaximumHealthPoints { maximum_health_points: 100 }, 
                        current_health_points: CurrentHealthPoints { current_health_points: 100 }, 
                        previous_current_health_points: Previous::from(CurrentHealthPoints { current_health_points: 100 }),
                        spawn: spawn.clone() 
                    }
                ).id();
                if monster_config.monster.beheadable {
                    commands.entity(entity).insert(Beheadable {});
                }
                monsters_map.map.insert(monster_id, entity);
            }
        }
    }
}

fn handle_position_added(player_query: Query<(&Position, &SocketWriter)>, monsters_query: Query<(&Id, &Monster, &Position, &CurrentHealthPoints, &MaximumHealthPoints), Added<Position>>) {
    for (player_position, socket_writer) in &player_query {
        for (id, monster, position, current_health_points, maximum_health_points) in &monsters_query {
            if position.is_in_sight(player_position) {
                let player_appear = MonsterAppearResponse::new(id, monster, position, current_health_points, maximum_health_points);
                socket_writer.write(&mut (&player_appear).into());
            }
        }
    }
}

fn handle_position_change(moved_query: Query<(&Previous<Position>, &Position, &SocketWriter), Changed<Position>>, monsters_query: Query<(&Id, &Monster, &Position, &CurrentHealthPoints, &MaximumHealthPoints)>) {
    for (moved_previous_position, moved_position, moved_socket_writer) in &moved_query {
        for (id, monster, position, current_health_points, maximum_health_points) in &monsters_query {
            if !position.is_in_sight(&moved_previous_position.entity) && position.is_in_sight(moved_position) {
                let player_appear = MonsterAppearResponse::new(id, monster, position, current_health_points, maximum_health_points);
                moved_socket_writer.write(&mut (&player_appear).into());
            }
        }
    }
}