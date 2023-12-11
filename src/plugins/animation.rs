use bevy::prelude::*;
use crate::components::animation::Animation;
use crate::components::network_writer::NetworkWriter;
use crate::components::network_observers::NetworkObservers;
use crate::components::id::Id;
use crate::components::normal_hit_animation::NormalHitAnimation;
use crate::components::skill_animation::SkillAnimation;
use crate::responses::animation::AnimationResponse;
use crate::responses::normal_hit_damage::NormalHitDamageResponse;
use crate::responses::skill_animation::SkillAnimationResponse;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, broadcast_animation);
        app.add_systems(Last, broadcast_skill_animation);
        app.add_systems(Last, broadcast_normal_hit_animation);
    }
}

fn broadcast_animation(
    mut commands: Commands, 
    query: Query<(Entity, &Id, &Animation, &NetworkObservers)>, 
    targets: Query<&Id>, 
    observers: Query<&NetworkWriter>
) {
     for (entity, id, animation, monster_observers) in &query {
        let target_id = if let Some(target_entity) = animation.target {
            if let Ok(id) = targets.get(target_entity) {
                Some(id.id)
            } else {
                None
            }
        } else {
            None
        };
        let animation_response = AnimationResponse { 
            player_id: id.id, 
            animation_index: animation.animation_index, 
            skill_index: animation.skill_index, 
            target_id
        };
        for entity in &monster_observers.entities {
            if let Ok(observer_socket_writer) = observers.get(*entity) {
                observer_socket_writer.write(&mut (&animation_response).into());
            }
        }
        commands.entity(entity).remove::<Animation>();
    }
}

fn broadcast_skill_animation(
    mut commands: Commands, 
    players: Query<(Entity, &Id, &SkillAnimation, &NetworkObservers)>, 
    targets: Query<&Id>, 
    observers: Query<&NetworkWriter>
) {
     for (entity, id, skill_animation, player_observers) in &players {
        if let Ok(target_id) = targets.get(skill_animation.target) {
            let skill_animation_response = SkillAnimationResponse { 
                skill_index: skill_animation.skill_index, 
                player_id: id.id, 
                target_id: target_id.id, 
                target_type: skill_animation.target_type, 
                animation_index: skill_animation.animation_index, 
                normal_damage: skill_animation.normal_damage, 
                explosive_blow_damage: skill_animation.explosive_blow_damage, 
                damage_type: skill_animation.damage_type, 
                soul_pocket_damage: skill_animation.soul_pocket_damage
            };
            for entity in &player_observers.entities {
                if let Ok(observer_socket_writer) = observers.get(*entity) {
                    observer_socket_writer.write(&mut (&skill_animation_response).into());
                }
            }
        }
        commands.entity(entity).remove::<SkillAnimation>();
    }
}

fn broadcast_normal_hit_animation(
    mut commands: Commands, 
    players: Query<(Entity, &Id, &NormalHitAnimation, &NetworkObservers)>, 
    targets: Query<&Id>, 
    observers: Query<&NetworkWriter>
) {
     for (entity, id, skill_animation, player_observers) in &players {
        if let Ok(target_id) = targets.get(skill_animation.target) {
            let skill_animation_response = NormalHitDamageResponse { 
                attacker_id: id.id, 
                target_id: target_id.id, 
                normal_damage: skill_animation.normal_damage, 
                explosive_blow_damage: skill_animation.explosive_blow_damage, 
                damage_type: skill_animation.damage_type, 
                soul_pocket_damage: skill_animation.soul_pocket_damage
            };
            for entity in &player_observers.entities {
                if let Ok(observer_socket_writer) = observers.get(*entity) {
                    observer_socket_writer.write(&mut (&skill_animation_response).into());
                }
            }
        }
        commands.entity(entity).remove::<NormalHitAnimation>();
    }
}