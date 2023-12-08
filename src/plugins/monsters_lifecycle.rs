use bevy::prelude::*;
use std::time::Duration;
use crate::components::behead_timer::BeheadTimer;
use crate::components::beheadable::Beheadable;
use crate::components::observers::Observers;
use crate::components::spawn::Spawn;
use crate::framework::entity_map::EntityMap;
use crate::components::id::Id;
use crate::configs::monsters::MonstersConfig;
use crate::bundles::monster::MonsterBundle;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::components::monster::Monster;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::framework::packet::Packet;

use super::tcp_server::SocketWriter;

pub struct MonstersLifecyclePlugin;

impl Plugin for MonstersLifecyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_monsters);
        app.add_systems(Update, tick_behead);
        app.add_systems(Update, tick_death);
        app.add_systems(PostUpdate, make_monsters_dead);
        app.add_systems(Update, tick_respawn);
        app.add_systems(Update, make_beheadable_monsters_dead);
        app.add_systems(Last, broadcast_death_animation);
        app.add_systems(Last, broadcast_knee_animation);
    }
}

#[derive(Component)]
struct Dead {
    timer: Timer
}

#[derive(Component)]
struct Respawn {
    timer: Timer
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
                        spawn: spawn.clone(),
                        observers: Observers::new()
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

fn make_beheadable_monsters_dead(mut commands: Commands, monsters_query: Query<(Entity, &CurrentHealthPoints), (Changed<CurrentHealthPoints>, With<Beheadable>)>) {
    for (entity, current_health_points) in &monsters_query {
        if current_health_points.current_health_points == 0 {
            commands.entity(entity).insert(BeheadTimer { timer: Timer::new(Duration::from_millis(5000), TimerMode::Once) });
        }
    }
}

fn make_monsters_dead(mut commands: Commands, monsters_query: Query<(Entity, &CurrentHealthPoints), (Changed<CurrentHealthPoints>, Without<Beheadable>)>) {
    for (entity, current_health_points) in &monsters_query {
        if current_health_points.current_health_points == 0 {
            commands.entity(entity).insert(Dead { timer: Timer::new(Duration::from_millis(5000), TimerMode::Once) });
        }
    }
}

fn tick_behead(mut commands: Commands, mut query: Query<(Entity, &mut BeheadTimer)>, time: Res<Time>) {
    for (entity, mut beheadable) in query.iter_mut() {
        beheadable.timer.tick(time.delta());
        if beheadable.timer.just_finished() {
            commands.entity(entity).insert(Dead { timer: Timer::new(Duration::from_millis(5000), TimerMode::Once) });
            commands.entity(entity).remove::<BeheadTimer>();
        }
    }
}

fn tick_death(mut commands: Commands, mut query: Query<(Entity, &mut Dead, &mut Position)>, time: Res<Time>) {
    for (entity, mut dead, mut position) in query.iter_mut() {
        dead.timer.tick(time.delta());
        if dead.timer.just_finished() {
            commands.entity(entity).remove::<Dead>();
            position.hide();
            commands.entity(entity).insert(Respawn { timer: Timer::new(Duration::from_millis(5000), TimerMode::Once) });
        }
    }
}

fn tick_respawn(mut commands: Commands, mut query: Query<(Entity, &Spawn, &mut Respawn, &mut CurrentHealthPoints, &MaximumHealthPoints, &mut Position)>, time: Res<Time>) {
    for (entity, spawn, mut respawn, mut current_health_points, maximum_health_points, mut position) in query.iter_mut() {
        respawn.timer.tick(time.delta());
        if respawn.timer.just_finished() {
            current_health_points.current_health_points = maximum_health_points.maximum_health_points;
            position.respawn(spawn);
            commands.entity(entity).remove::<Respawn>();
        }
    }
}

fn broadcast_death_animation(monsters: Query<(&Id, &Observers), Added<Dead>>, observers: Query<&SocketWriter>) {
    for (monster_id, monster_observers) in &monsters {
        for entity in &monster_observers.entities {
            if let Ok(observer_socket_writer) = observers.get(*entity) {
                let mut response = Packet::from(61);
                response.write_i32(monster_id.id);
                response.write_buffer(&[9]); // 10 = behead  - 8 = knee
                observer_socket_writer.write(&mut response);
            }
        }
    }
}

fn broadcast_knee_animation(monsters: Query<(&Id, &Observers), Added<BeheadTimer>>, observers: Query<&SocketWriter>) {
     for (monster_id, monster_observers) in &monsters {
        for entity in &monster_observers.entities {
            if let Ok(observer_socket_writer) = observers.get(*entity) {
                let mut response = Packet::from(61);
                response.write_i32(monster_id.id);
                response.write_buffer(&[8]); // 8 = bh
                observer_socket_writer.write(&mut response);
            }
        }
    }
}
