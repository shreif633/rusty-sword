use bevy::prelude::*;
use std::time::Duration;
use crate::components::aggro::Aggro;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::dead::Dead;
use crate::components::experience::Experience;
use crate::components::id::Id;
use crate::components::monster::Monster;
use crate::components::observers::Observers;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::walking::Walking;
use crate::enums::damage_type::DamageType;
use crate::framework::entity_map::EntityMap;
use crate::requests::normal_hit::NormalHitRequest;
use crate::responses::normal_hit_damage::NormalHitDamageResponse;
use crate::responses::player_experience::PlayerExperienceResponse;
use super::tcp_server::SocketWriter;

pub struct NormalHitPlugin;

impl Plugin for NormalHitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, stop_attacking_when_moving);
        app.add_systems(Update, handle_normal_hit);
        app.add_systems(Update, tick_normal_hit);
        app.add_systems(PostUpdate, defend_damage);
        app.add_systems(Last, calculate_damage);
        app.add_systems(Update, distribute_experience);
    }
}

#[derive(Component)]
struct NormalHitTarget {
    pub timer: Timer,
    pub target: Entity
}

fn handle_normal_hit(mut commands: Commands, mut players_query: Query<(Entity, &Position, &NormalHitRequest, Option<&mut NormalHitTarget>)>, monsters_query: Query<(Entity, &Position), With<Monster>>, monsters_map: Res<EntityMap<Monster>>) {
    for (entity, position, client_packet, normal_hit_target) in players_query.iter_mut() {
        if let Some(monster_entity) = monsters_map.map.get(&client_packet.target_id) {
            if let Ok((monster_entity, monster_position)) = monsters_query.get(*monster_entity) {
                if let Some(mut normal_hit_target) = normal_hit_target {
                    if normal_hit_target.target != monster_entity {
                        normal_hit_target.target = monster_entity;
                        normal_hit_target.timer.reset();
                    }
                } else if monster_position.is_in_sight(position) {
                    commands.entity(entity).insert(NormalHitTarget { target: monster_entity, timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating) });
                }
            }
        }
        commands.entity(entity).remove::<NormalHitRequest>();
    }
}

enum DamageNature {
    Normal
}

#[derive(Component)]
struct Damage {
    source: Entity,
    target: Entity,
    damage: u32,
    aggro_multiplier: f32,
    nature: DamageNature
}

fn tick_normal_hit(mut commands: Commands, mut attackers: Query<(Entity, &mut NormalHitTarget, &Position), With<Player>>, mut targets: Query<(Entity, &Position), With<Monster>>, time: Res<Time>) {
    for (entity, mut normal_hit_target, position) in attackers.iter_mut() {
        normal_hit_target.timer.tick(time.delta());
        if normal_hit_target.timer.just_finished() {
            if let Ok((monster_entity, monster_position)) = targets.get_mut(normal_hit_target.target) {
                let damage = 10;
                if monster_position.is_in_sight(position) {
                    commands.spawn(Damage { source: entity, target: monster_entity, damage, aggro_multiplier: 1.0, nature: DamageNature::Normal });
                }
            }
        }
    }
}

fn defend_damage(mut damages: Query<&mut Damage, Added<Damage>>) {
    for mut damage in damages.iter_mut() {
        damage.damage -= 2;
    }
}

fn calculate_damage(mut commands: Commands, damages: Query<(Entity, &Damage), Added<Damage>>, mut targets: Query<(&Id, &mut CurrentHealthPoints, &Observers, Option<&mut Aggro>)>, attackers: Query<&Id>, observers: Query<&SocketWriter>) {
    for (damage_entity, damage) in damages.iter() {
        if let Ok((target_id, mut target_current_health_points, target_observers, optional_aggro)) = targets.get_mut(damage.target) {
            if let Ok(attacker_id) = attackers.get(damage.source) {
                target_current_health_points.sub(damage.damage);
                if target_current_health_points.current_health_points == 0 {
                    commands.entity(damage.source).remove::<NormalHitTarget>();
                }
                if let Some(mut aggro) = optional_aggro {
                    let total_aggro: u32 = (damage.damage as f32 * damage.aggro_multiplier) as u32;
                    aggro.add(damage.source, total_aggro);
                }
                commands.entity(damage_entity).despawn();
                let normal_hit_damage_response = NormalHitDamageResponse {
                    attacker_id: attacker_id.id,
                    target_id: target_id.id,
                    normal_damage: damage.damage,
                    explosive_blow_damage: 0,
                    damage_type: DamageType::Normal,
                    soul_pocket_damage: 0,
                };
                for entity in &target_observers.entities {
                    if let Ok(observer_socket_writer) = observers.get(*entity) {
                        observer_socket_writer.write(&mut (&normal_hit_damage_response).into());
                    }
                }
            }
        }
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

fn stop_attacking_when_moving(mut commands: Commands, query: Query<Entity, (With<Walking>, With<NormalHitTarget>)>) {
    for entity in &query {
        commands.entity(entity).remove::<NormalHitTarget>();
    }
}

// mob aggro
// mob walk
// allot exp by color
// validate distance
// attack the one with most aggro
// remove client packet on end of frame
// rest
// monster follow
// level up