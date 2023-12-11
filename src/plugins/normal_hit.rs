use bevy::prelude::*;
use std::time::Duration;
use crate::components::damage::Damage;
use crate::components::dead::Dead;
use crate::components::monster::Monster;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::walking::Walking;
use crate::framework::entity_map::EntityMap;
use crate::requests::normal_hit::NormalHitRequest;

pub struct NormalHitPlugin;

impl Plugin for NormalHitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, stop_attacking_when_moving);
        app.add_systems(PreUpdate, stop_attacking_when_enemy_dies);
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

fn tick_normal_hit(mut commands: Commands, mut attackers: Query<(Entity, &mut NormalHitTarget, &Position), With<Player>>, mut targets: Query<(Entity, &Position), With<Monster>>, time: Res<Time>) {
    for (entity, mut normal_hit_target, position) in attackers.iter_mut() {
        normal_hit_target.timer.tick(time.delta());
        if normal_hit_target.timer.just_finished() {
            if let Ok((monster_entity, monster_position)) = targets.get_mut(normal_hit_target.target) {
                let damage = 50;
                if monster_position.is_in_sight(position) {
                    let damage = Damage { 
                        source: entity, 
                        target: monster_entity, 
                        damage, 
                        aggro_multiplier: 1.0,
                        skill_index: None,
                        animation: None
                    };
                    commands.spawn(damage);
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

fn stop_attacking_when_enemy_dies(
    mut commands: Commands, 
    query: Query<(Entity, &NormalHitTarget)>,
    targets: Query<&Dead>
) {
    for (entity, skill) in &query {
        if targets.get(skill.target).is_ok() {
            commands.entity(entity).remove::<NormalHitTarget>();
        }
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

// CChar::WriteInSight(this->GetOffset(), 10, "bbddddd", GetType, SkillID, this->GetID(), Target.GetID(), MaxDmg, EBDamage, 0);

// [response] Unknown(Packet { buffer: [25, 0, 10, 1, 24, 234, 120, 0, 0, 113, 93, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], cursor: 3 })
// [response] Unknown(Packet { buffer: [25, 0, 10, 1, 24, 234, 120, 0, 0, 116, 93, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], cursor: 3 })
// [response] Unknown(Packet { buffer: [25, 0, 10, 1, 24, 234, 120, 0, 0, 116, 93, 0, 0, 23, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], cursor: 3 })

// pub const HEADER: u8 = 10;

// #[derive(Debug)]
// pub struct SkillExecuteResponse {
//     pub target_type: TargetType,
//     pub skill_index: u8,
//     pub player_id: i32,
//     pub target_id: i32,
//     // pub unknown: u8,
//     pub normal_damage: Option<u16>,
//     pub explosive_blow_damage: Option<u16>,
//     // pub damage_type: Option<DamageType>,
//     pub soul_pocket_damage: Option<u16>
// }