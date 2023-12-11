use bevy::prelude::*;
use std::time::Duration;
use crate::components::animation::Animation;
use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::damage::Damage;
use crate::components::dead::Dead;
use crate::components::monster::Monster;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::walking::Walking;
use crate::framework::entity_map::EntityMap;
use crate::requests::skill_prepare::SkillPrepareRequest;

const MANA_COST: u16 = 150;

pub struct StaggeringBlowPlugin;

impl Plugin for StaggeringBlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, stop_attacking_when_moving);
        app.add_systems(PreUpdate, stop_attacking_when_enemy_dies);
        app.add_systems(PreUpdate, stop_attacking_when_mana_ends);
        app.add_systems(Update, handle_skill_prepare);
        app.add_systems(Update, tick_cast_timer);
        app.add_systems(Update, tick_continue_timer);
    }
}

#[derive(Component)]
struct StaggeringBlowSkill {
    target: Entity,
    cast_timer: Timer,
    continue_timer: Timer,
    casting: bool,
}

fn handle_skill_prepare(
    mut commands: Commands, 
    query: Query<(Entity, &SkillPrepareRequest, Option<&StaggeringBlowSkill>)>,
    monsters_map: Res<EntityMap<Monster>>
) {
    for (entity, client_packet, optional_skill) in &query {
        println!("{:?}", client_packet);
        if client_packet.skill_index == 3 {
            if let Some(monster_entity) = monsters_map.map.get(&client_packet.target_id) {
                if optional_skill.is_none() || optional_skill.unwrap().target != *monster_entity {
                    commands.entity(entity).insert(
                        StaggeringBlowSkill { 
                            target: *monster_entity, 
                            cast_timer: Timer::new(Duration::from_millis(2000), TimerMode::Repeating),
                            continue_timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
                            casting: false
                        }
                    );
                }               
            }
        } else {
            commands.entity(entity).remove::<StaggeringBlowSkill>();
        }
    }
}

fn tick_cast_timer(
    mut commands: Commands, 
    mut attackers: Query<(Entity, &mut StaggeringBlowSkill, &Position, &mut CurrentMagicPoints), With<Player>>, 
    mut targets: Query<(Entity, &Position), With<Monster>>,
    time: Res<Time>
) {
    for (entity, mut staggering_blow_skill, position, mut current_magic_points) in attackers.iter_mut() {
        if staggering_blow_skill.casting {
            staggering_blow_skill.cast_timer.tick(time.delta());
            if staggering_blow_skill.cast_timer.just_finished() {
                if let Ok((monster_entity, monster_position)) = targets.get_mut(staggering_blow_skill.target) {
                    let damage = 50;
                    if monster_position.is_in_sight(position) {
                        current_magic_points.current_magic_points -= MANA_COST;
                        let damage = Damage { 
                            source: entity, 
                            target: monster_entity, 
                            damage: damage as u32, 
                            aggro_multiplier: 1.0 ,
                            skill_index: Some(3),
                            animation: Some(10)
                        };
                        commands.spawn(damage);
                    }
                }
                staggering_blow_skill.casting = false;
            }
        }
    }
}

fn tick_continue_timer(
    mut commands: Commands, 
    mut attackers: Query<(Entity, &mut StaggeringBlowSkill, &Position), With<Player>>, 
    mut targets: Query<(Entity, &Position), With<Monster>>,
    time: Res<Time>
) {
    for (entity, mut staggering_blow_skill, position) in attackers.iter_mut() {
        if !staggering_blow_skill.casting {
            let just_started = staggering_blow_skill.continue_timer.elapsed_secs() == 0.0;
            staggering_blow_skill.continue_timer.tick(time.delta());
            if staggering_blow_skill.continue_timer.just_finished() || just_started {
                if let Ok((monster_entity, monster_position)) = targets.get_mut(staggering_blow_skill.target) {
                    if monster_position.is_in_sight(position) {
                        commands.entity(entity).insert(Animation::with_target(5, 3, monster_entity));
                        staggering_blow_skill.casting = true;
                    }
                }
            }
        }
    }
}

fn stop_attacking_when_moving(mut commands: Commands, query: Query<Entity, (With<Walking>, With<StaggeringBlowSkill>)>) {
    for entity in &query {
        commands.entity(entity).remove::<StaggeringBlowSkill>();
    }
}

fn stop_attacking_when_mana_ends(mut commands: Commands, query: Query<(Entity, &CurrentMagicPoints), With<StaggeringBlowSkill>>) {
    for (entity, current_magic_points) in &query {
        if current_magic_points.current_magic_points < MANA_COST {
            commands.entity(entity).remove::<StaggeringBlowSkill>();
        }
    }
}

fn stop_attacking_when_enemy_dies(
    mut commands: Commands, 
    query: Query<(Entity, &StaggeringBlowSkill)>,
    targets: Query<With<Dead>>
) {
    for (entity, skill) in &query {
        if targets.get(skill.target).is_ok() {
            commands.entity(entity).remove::<StaggeringBlowSkill>();
        }
    }
}

