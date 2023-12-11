use bevy::prelude::*;
use crate::components::aggro::Aggro;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::damage::Damage;
use crate::components::id::Id;
use crate::components::observers::Observers;
use crate::components::skill_animation::SkillAnimation;
use crate::enums::damage_type::DamageType;
use crate::enums::target_type::TargetType;
use crate::responses::normal_hit_damage::NormalHitDamageResponse;
use super::tcp_server::SocketWriter;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, defend_damage);
        app.add_systems(Last, calculate_aggro.before(clear_damage));
        app.add_systems(Last, decrease_health.before(clear_damage));
        app.add_systems(Last, add_damage_animation.before(clear_damage));
        app.add_systems(Last, clear_damage);
    }
}

fn defend_damage(mut damages: Query<&mut Damage, Added<Damage>>) {
    for mut damage in damages.iter_mut() {
        damage.damage -= 2;
    }
}

fn decrease_health(damages: Query<&Damage, Added<Damage>>, mut targets: Query<&mut CurrentHealthPoints>) {
    for damage in damages.iter() {
        if let Ok(mut target_current_health_points) = targets.get_mut(damage.target) {
            target_current_health_points.sub(damage.damage);
        }
    }
}

fn calculate_aggro(damages: Query<&Damage, Added<Damage>>, mut targets: Query<&mut Aggro>) {
    for damage in damages.iter() {
        if let Ok(mut aggro) = targets.get_mut(damage.target) {
            let total_aggro: u32 = (damage.damage as f32 * damage.aggro_multiplier) as u32;
            aggro.add(damage.source, total_aggro);
        }
    }
}

fn add_damage_animation(
    mut commands: Commands, 
    damages: Query<&Damage, Added<Damage>>, 
    targets: Query<(&Id, &Observers)>, 
    attackers: Query<&Id>, 
    observers: Query<&SocketWriter>
) {
    for damage in damages.iter() {
        if let Ok((target_id, target_observers)) = targets.get(damage.target) {
            if let Ok(attacker_id) = attackers.get(damage.source) {
                if let Some(animation) = damage.animation {
                    if let Some(skill_index) = damage.skill_index {
                        let skill_animation = SkillAnimation::with_damage(skill_index, damage.target, TargetType::Monster, animation, damage.damage as u16, 0, DamageType::Normal, 0);
                        commands.entity(damage.source).insert(skill_animation);
                    }   
                } else {
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
}

fn clear_damage(mut commands: Commands, damages: Query<Entity, With<Damage>>) {
    for damage_entity in damages.iter() {
        commands.entity(damage_entity).despawn();
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