use bevy::prelude::*;
use crate::components::animation::Animation;
use crate::components::behead_timer::BeheadTimer;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::maximum_magic_points::MaximumMagicPoints;
use crate::components::monster::Monster;
use crate::components::position::Position;
use crate::components::skill_animation::SkillAnimation;
use crate::enums::target_type::TargetType;
use crate::framework::entity_map::EntityMap;
use crate::requests::skill_execute::SkillExecuteRequest;

pub struct BeheadPlugin;

impl Plugin for BeheadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_behead_skill);
        app.add_systems(Update, handle_skill_execute);
    }
}

#[derive(Component)]
struct BeheadSkill {
    target: Entity,
}

fn handle_skill_execute(mut commands: Commands, query: Query<(Entity, &SkillExecuteRequest, &Position)>, monsters_query: Query<(Entity, &Position), With<BeheadTimer>>, monsters_map: Res<EntityMap<Monster>>) {
    for (entity, client_packet, position) in &query {
        if client_packet.skill_index == 1 {
            if let Some(target_id) = client_packet.target_id {
                if let Some(monster_entity) = monsters_map.map.get(&target_id) {
                    if let Ok((monster_entity, monster_position)) = monsters_query.get(*monster_entity) {
                        if position.is_in_range(monster_position, 45) {
                            commands.entity(entity).insert(BeheadSkill { target: monster_entity });
                        }
                    }
                }
            }
        }
    }
}

fn handle_behead_skill(
    mut commands: Commands,
    mut players: Query<(Entity, &BeheadSkill, &mut CurrentHealthPoints, &MaximumHealthPoints, &mut CurrentMagicPoints, &MaximumMagicPoints)>,
    monsters: Query<Entity, With<BeheadTimer>>,
) {
    for (behead_entity, behead_skills, mut current_health_points, maximum_health_points, mut current_magic_points, maximum_magic_points) in players.iter_mut() {
        if let Ok(monster_entity) = monsters.get(behead_skills.target) {
            current_health_points.current_health_points += maximum_health_points.maximum_health_points / 10;
            current_magic_points.current_magic_points += maximum_magic_points.maximum_magic_points / 10;
            commands.entity(monster_entity).insert(Animation::without_target(10));
            commands.entity(behead_entity).insert(SkillAnimation::without_damage(1, monster_entity, TargetType::Monster, 1));
        }
        commands.entity(behead_entity).remove::<BeheadSkill>();
    }
}