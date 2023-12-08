use std::time::Duration;

use bevy::prelude::*;
use crate::components::beheadable::Beheadable;
use crate::components::current_health_points::{CurrentHealthPoints, self};
use crate::components::id::Id;
use crate::components::maximum_health_points::{MaximumHealthPoints, self};
use crate::components::monster::Monster;
use crate::components::player::Player;
use crate::components::position::{Position, self};
use crate::components::spawn::Spawn;
use crate::components::walking::Walking;
use crate::enums::damage_type::DamageType;
use crate::enums::target_type::TargetType;
use crate::framework::entity_map::EntityMap;
use crate::framework::packet::Packet;
use crate::requests::normal_hit::NormalHitRequest;
use crate::requests::skill_execute::SkillExecuteRequest;
use crate::requests::skill_prepare::SkillPrepareRequest;
use crate::responses::normal_hit_damage::NormalHitDamageResponse;
use crate::responses::skill_execute::SkillExecuteResponse;
use super::tcp_server::SocketWriter;

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, stop_attacking_when_moving);
        app.add_systems(Update, handle_normal_hit);
        app.add_systems(Update, tick_normal_hit);
        app.add_systems(Update, tick_behead);
        app.add_systems(Update, tick_death);
        app.add_systems(PostUpdate, make_monsters_dead);
        app.add_systems(Last, broadcast_behead_animation);
        app.add_systems(Last, broadcast_death_animation);
        app.add_systems(Last, broadcast_disappear);
        app.add_systems(Update, tick_respawn);
        app.add_systems(Update, tick_disappear);
        app.add_systems(Update, make_beheadable_monsters_dead);
        app.add_systems(Update, handle_behead_skill);
        app.add_systems(Update, handle_skill_prepare);
        app.add_systems(Update, handle_skill_execute);
    }
}

#[derive(Component)]
struct NormalHitTarget {
    pub timer: Timer,
    pub target: Entity
}

fn handle_normal_hit(mut commands: Commands, mut players_query: Query<(Entity, &Position, &NormalHitRequest, Option<&mut NormalHitTarget>)>, monsters_query: Query<(Entity, &Position), With<Monster>>, monsters_map: Res<EntityMap<Monster>>) {
    for (entity, position, client_packet, normal_hit_target) in players_query.iter_mut() {
        println!("client_packet: {:?}", client_packet);
        if let Some(monster_entity) = monsters_map.map.get(&client_packet.target_id) {
            println!("monster_entity: {:?}", monster_entity);
            if let Ok((monster_entity, monster_position)) = monsters_query.get(*monster_entity) {
                if let Some(mut normal_hit_target) = normal_hit_target {
                    if normal_hit_target.target != monster_entity {
                        normal_hit_target.target = monster_entity;
                        normal_hit_target.timer.reset();
                    }
                } else {
                    println!("monster_entity: {:?}", monster_entity);
                    if monster_position.is_in_sight(position) {
                        println!("in sight");
                        commands.entity(entity).insert(NormalHitTarget { target: monster_entity, timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating) });
                    }
                }
            }
        }
        commands.entity(entity).remove::<NormalHitRequest>();
    }
}

fn tick_normal_hit(mut commands: Commands, mut query: Query<(Entity, &Id, &mut NormalHitTarget, &Position, &SocketWriter), With<Player>>, mut monsters_query: Query<(&Id, &Position, &mut CurrentHealthPoints), With<Monster>>, time: Res<Time>) {
    for (entity, id, mut normal_hit_target, position, socket_writer) in query.iter_mut() {
        normal_hit_target.timer.tick(time.delta());
        if normal_hit_target.timer.just_finished() {
            if let Ok((monster_id, monster_position, mut monster_current_health_points)) = monsters_query.get_mut(normal_hit_target.target) {
                let damage = 10;
                monster_current_health_points.sub(damage);
                if monster_position.is_in_sight(position) {
                    let response = NormalHitDamageResponse {
                        attacker_id: id.id,
                        target_id: monster_id.id,
                        normal_damage: damage,
                        explosive_blow_damage: 0,
                        damage_type: DamageType::Normal,
                        soul_pocket_damage: 0,
                    };
                    let mut packet = Packet::from(&response);
                    socket_writer.write(&mut packet);
                }
                if monster_current_health_points.current_health_points == 0 {
                    commands.entity(entity).remove::<NormalHitTarget>();
                }
            }
        }
    }
}

fn stop_attacking_when_moving(mut commands: Commands, query: Query<Entity, (With<Walking>, With<NormalHitTarget>)>) {
    for entity in &query {
        commands.entity(entity).remove::<NormalHitTarget>();
    }
}

fn make_beheadable_monsters_dead(mut commands: Commands, monsters_query: Query<(Entity, &CurrentHealthPoints), (Changed<CurrentHealthPoints>, With<Beheadable>)>) {
    for (entity, current_health_points) in &monsters_query {
        if current_health_points.current_health_points <= 0 {
            commands.entity(entity).insert(BeheadTimer { timer: Timer::new(Duration::from_millis(5000), TimerMode::Once) });
        }
    }
}

fn make_monsters_dead(mut commands: Commands, monsters_query: Query<(Entity, &CurrentHealthPoints), (Changed<CurrentHealthPoints>, Without<Beheadable>)>) {
    for (entity, current_health_points) in &monsters_query {
        if current_health_points.current_health_points <= 0 {
            commands.entity(entity).insert(Dead { timer: Timer::new(Duration::from_millis(5000), TimerMode::Once) });
        }
    }
}

#[derive(Component)]
struct BeheadTimer {
    timer: Timer
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

fn broadcast_death_animation(monsters_query: Query<(&Id, &Position), Added<Dead>>, observers_query: Query<(&Position, &SocketWriter)>) {
    for (id, position) in &monsters_query {
        for (observer_position, observer_socket_writer) in &observers_query {
            println!("BROADCAST DEATH");
            if position.is_in_sight(observer_position) {
                let mut response = Packet::from(61);
                response.write_i32(id.id);
                response.write_buffer(&vec![9]); // 10 = behead  - 8 = knee
                observer_socket_writer.write(&mut response);
            }
        }
    }
}

#[derive(Component)]
struct Dead {
    timer: Timer
}

#[derive(Component)]
struct Disappear {
    timer: Timer
}

#[derive(Component)]
struct Respawn {
    timer: Timer
}

fn tick_death(mut commands: Commands, mut query: Query<(Entity, &mut Dead)>, time: Res<Time>) {
    for (entity, mut dead) in query.iter_mut() {
        dead.timer.tick(time.delta());
        if dead.timer.just_finished() {
            println!("DEAD TICKED");
            commands.entity(entity).remove::<Dead>();
            commands.entity(entity).insert(Disappear { timer: Timer::new(Duration::from_millis(5000), TimerMode::Once) });
        }
    }
}

fn broadcast_disappear(monsters_query: Query<(&Id, &Position), Added<Disappear>>, observers_query: Query<(&Position, &SocketWriter)>) {
    for (id, position) in &monsters_query {
        for (observer_position, observer_socket_writer) in &observers_query {
            println!("BROADCAST DISAPPEAR");
            if position.is_in_sight(observer_position) {
                let mut response = Packet::from(57);
                response.write_i32(id.id);
                observer_socket_writer.write(&mut response);
            }
        }
    }
}

fn tick_disappear(mut commands: Commands, mut query: Query<(Entity, &mut Disappear)>, time: Res<Time>) {
    for (entity, mut disappear) in query.iter_mut() {
        disappear.timer.tick(time.delta());
        if disappear.timer.just_finished() {
            println!("Disappear TICKED");
            commands.entity(entity).remove::<Disappear>();
            commands.entity(entity).remove::<Position>();
            commands.entity(entity).insert(Respawn { timer: Timer::new(Duration::from_millis(5000), TimerMode::Once) });
        }
    }
}

fn tick_respawn(mut commands: Commands, mut query: Query<(Entity, &Spawn, &mut Respawn, &mut CurrentHealthPoints, &MaximumHealthPoints)>, time: Res<Time>) {
    for (entity, spawn, mut respawn, mut current_health_points, maximum_health_points) in query.iter_mut() {
        respawn.timer.tick(time.delta());
        if respawn.timer.just_finished() {
            println!("ADD RESPAWN");
            current_health_points.current_health_points = maximum_health_points.maximum_health_points;
            commands.entity(entity).remove::<Respawn>();
            commands.entity(entity).insert(Position::from(spawn));
        }
    }
}

fn broadcast_behead_animation(monsters_query: Query<(&Id, &Position), Added<BeheadTimer>>, observers_query: Query<(&Position, &SocketWriter)>) {
    for (id, position) in &monsters_query {
        for (observer_position, observer_socket_writer) in &observers_query {
            println!("BROADCAST BEHEAD");
            if position.is_in_sight(observer_position) {
                let mut response = Packet::from(61);
                response.write_i32(id.id);
                response.write_buffer(&vec![8]); // 8 = bh
                observer_socket_writer.write(&mut response);
            }
        }
    }
}

#[derive(Component)]
struct SkillBehead {
    from: Entity,
    to: Entity,
}

fn handle_skill_prepare(mut commands: Commands, query: Query<(Entity, &SkillPrepareRequest, &Position)>, monsters_query: Query<(Entity, &Position), With<BeheadTimer>>, monsters_map: Res<EntityMap<Monster>>) {
    for (entity, client_packet, position) in &query {
        println!("SKILL PREPARE {:?}", client_packet);
        if client_packet.skill_index == 1 {
            if let Some(monster_entity) = monsters_map.map.get(&client_packet.target_id) {
                if let Ok((monster_entity, monster_position)) = monsters_query.get(*monster_entity) {
                    if position.is_in_sight(monster_position) {
                        println!("DISTANCE: {}", position.calculate_distance(monster_position));
                        commands.spawn(SkillBehead { from: entity, to: monster_entity });
                    }
                }
            }
        }
        commands.entity(entity).remove::<SkillPrepareRequest>();
    }
}

fn handle_skill_execute(mut commands: Commands, query: Query<(Entity, &SkillExecuteRequest, &Position)>, monsters_query: Query<(Entity, &Position), With<BeheadTimer>>, monsters_map: Res<EntityMap<Monster>>) {
    for (entity, client_packet, position) in &query {
        println!("SKILL PREPARE {:?}", client_packet);
        if client_packet.skill_index == 1 {
            if let Some(target_id) = client_packet.target_id {
                if let Some(monster_entity) = monsters_map.map.get(&target_id) {
                    if let Ok((monster_entity, monster_position)) = monsters_query.get(*monster_entity) {
                        if position.is_in_sight(monster_position) {
                            println!("DISTANCE: {}", position.calculate_distance(monster_position));
                            commands.spawn(SkillBehead { from: entity, to: monster_entity });
                        }
                    }
                }
            }
        }
        commands.entity(entity).remove::<SkillExecuteRequest>();
    }
}

fn handle_behead_skill(
    mut commands: Commands,
    behead_skills: Query<(Entity, &SkillBehead)>,
    mut players: Query<(&Id, &mut CurrentHealthPoints, &MaximumHealthPoints, &SocketWriter)>,
    monsters: Query<&Id, With<BeheadTimer>>,
) {
    for (behead_entity, behead_skills) in &behead_skills {
        if let Ok(monster_id) = monsters.get(behead_skills.to) {
            if let Ok((player_id, mut current_health_points, maximum_health_points, socket_writer)) = players.get_mut(behead_skills.from) {
                current_health_points.current_health_points += maximum_health_points.maximum_health_points / 10;
                let mut response = Packet::from(61);
                response.write_i32(monster_id.id);
                response.write_buffer(&vec![10]); // 8 = bh
                socket_writer.write(&mut response);

                let skill_execute_response = SkillExecuteResponse { 
                    skill_index: 1, 
                    player_id: player_id.id, 
                    target_id: monster_id.id, 
                    target_type: TargetType::Player, 
                    unknown: 1, 
                    normal_damage: None, 
                    explosive_blow_damage: None, 
                    damage_type: None, 
                    soul_pocket_damage: None 
                };
                socket_writer.write(&mut (&skill_execute_response).into());
            }
        }
        println!("BEHEADEDEDEDED");
        commands.entity(behead_entity).despawn();
    }
}

// mob aggro
// mob walk
// allot exp
// behead skill
// run 
// sight
// animate behead


// movement
// let mut response = Packet::from(36);
// response.write_i32(client_packet.target_id);
// response.write_buffer(&vec![18, 25, 1]);

// idk
// let mut response = Packet::from(117);
// response.write_buffer(&vec![1, 1]);
// response.write_i32(client_packet.target_id);
// socket_writer.write(&mut response);

// set experience
// let mut response = Packet::from(69);
// response.write_buffer(&vec![25, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);

// let mut response = Packet::from(69);
// response.write_buffer(&vec![25, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);

// let mut packet = Packet::from(62);
// packet.write_i32(client_packet.target_id);
// packet.write_i32(id.id);
// packet.write_u32(10);
// packet.write_u32(5);
// packet.write_u8(1);
// packet.write_u32(3);