use bevy::prelude::*;
use crate::components::behead_timer::BeheadTimer;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::id::Id;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::maximum_magic_points::MaximumMagicPoints;
use crate::components::monster::Monster;
use crate::components::position::Position;
use crate::enums::target_type::TargetType;
use crate::framework::entity_map::EntityMap;
use crate::framework::packet::Packet;
use crate::requests::skill_execute::SkillExecuteRequest;
use crate::requests::skill_prepare::SkillPrepareRequest;
use crate::responses::skill_execute::SkillExecuteResponse;
use super::tcp_server::SocketWriter;

pub struct BeheadPlugin;

impl Plugin for BeheadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_behead_skill);
        app.add_systems(Update, handle_skill_prepare);
        app.add_systems(Update, handle_skill_execute);
        app.add_systems(Last, clear_skill_execute);
        app.add_systems(Last, clear_skill_prepare);
    }
}

#[derive(Component)]
struct SkillBehead {
    from: Entity,
    to: Entity,
}

fn handle_skill_prepare(mut commands: Commands, query: Query<(Entity, &SkillPrepareRequest, &Position)>, monsters_query: Query<(Entity, &Position), With<BeheadTimer>>, monsters_map: Res<EntityMap<Monster>>) {
    for (entity, client_packet, position) in &query {
        if client_packet.skill_index == 1 {
            if let Some(monster_entity) = monsters_map.map.get(&client_packet.target_id) {
                if let Ok((monster_entity, monster_position)) = monsters_query.get(*monster_entity) {
                    if position.is_in_sight(monster_position) {
                        commands.spawn(SkillBehead { from: entity, to: monster_entity });
                    }
                }
            }
        }
    }
}

fn clear_skill_prepare(mut commands: Commands, query: Query<Entity, With<SkillPrepareRequest>>) {
    for entity in &query {
        commands.entity(entity).remove::<SkillPrepareRequest>();
    }
}

fn clear_skill_execute(mut commands: Commands, query: Query<Entity, With<SkillExecuteRequest>>) {
    for entity in &query {
        commands.entity(entity).remove::<SkillExecuteRequest>();
    }
}

fn handle_skill_execute(mut commands: Commands, query: Query<(Entity, &SkillExecuteRequest, &Position)>, monsters_query: Query<(Entity, &Position), With<BeheadTimer>>, monsters_map: Res<EntityMap<Monster>>) {
    for (entity, client_packet, position) in &query {
        if client_packet.skill_index == 1 {
            if let Some(target_id) = client_packet.target_id {
                if let Some(monster_entity) = monsters_map.map.get(&target_id) {
                    if let Ok((monster_entity, monster_position)) = monsters_query.get(*monster_entity) {
                        if position.is_in_range(monster_position, 45) {
                            commands.spawn(SkillBehead { from: entity, to: monster_entity });
                        }
                    }
                }
            }
        }
    }
}

fn handle_behead_skill(
    mut commands: Commands,
    behead_skills: Query<(Entity, &SkillBehead)>,
    mut players: Query<(&Id, &mut CurrentHealthPoints, &MaximumHealthPoints, &mut CurrentMagicPoints, &MaximumMagicPoints, &SocketWriter)>,
    monsters: Query<&Id, With<BeheadTimer>>,
) {
    for (behead_entity, behead_skills) in &behead_skills {
        if let Ok(monster_id) = monsters.get(behead_skills.to) {
            if let Ok((player_id, mut current_health_points, maximum_health_points, mut current_magic_points, maximum_magic_points, socket_writer)) = players.get_mut(behead_skills.from) {
                current_health_points.current_health_points += maximum_health_points.maximum_health_points / 10;
                current_magic_points.current_magic_points += maximum_magic_points.maximum_magic_points / 10;
                let mut response = Packet::from(61);
                response.write_i32(monster_id.id);
                response.write_buffer(&[10]); // 8 = bh
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
        commands.entity(behead_entity).despawn();
    }
}