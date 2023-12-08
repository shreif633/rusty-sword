use bevy::prelude::*;
use std::time::Duration;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::id::Id;
use crate::components::monster::Monster;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::walking::Walking;
use crate::enums::damage_type::DamageType;
use crate::framework::entity_map::EntityMap;
use crate::framework::packet::Packet;
use crate::requests::normal_hit::NormalHitRequest;
use crate::responses::normal_hit_damage::NormalHitDamageResponse;
use super::tcp_server::SocketWriter;

pub struct NormalHitPlugin;

impl Plugin for NormalHitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, stop_attacking_when_moving);
        app.add_systems(Update, handle_normal_hit);
        app.add_systems(Update, tick_normal_hit);
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

// mob aggro
// mob walk
// allot exp
// run 
// validate distance