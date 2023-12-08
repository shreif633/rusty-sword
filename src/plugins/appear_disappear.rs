use bevy::prelude::*;
use crate::components::appearance::Appearance;
use crate::components::npc::Npc;
use crate::components::observers::Observers;
use crate::components::player::Player;
use crate::responses::monster_disappear::MonsterDisappearResponse;
use crate::components::id::Id;
use crate::components::position::Position;
use crate::responses::monster_appear::MonsterAppearResponse;
use crate::components::monster::Monster;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::direction::Direction;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::responses::npc_appear::NpcAppearResponse;
use crate::responses::player_appear::PlayerAppearResponse;
use crate::responses::player_disappear::PlayerDisappearResponse;
use super::tcp_server::SocketWriter;

pub struct AppearDisappearPlugin;

impl Plugin for AppearDisappearPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_observers_list);
        app.add_systems(Update, broadcast_appear);
        app.add_systems(Update, broadcast_disappear);
        app.add_event::<AppearEvent>();
        app.add_event::<DisappearEvent>();
    }
}

#[derive(Event)]
struct AppearEvent {
    entity: EntityType,
    observer: Entity
}

enum EntityType {
    Player(Entity),
    Monster(Entity),
    Npc(Entity)
}

#[derive(Event)]
struct DisappearEvent {
    entity: EntityType,
    observer: Entity
}

fn update_observers_list(mut monsters: Query<(Entity, &mut Observers, &Position, Option<&Monster>, Option<&Player>)>, players: Query<(Entity, &Position), With<Player>>, mut appear_event: EventWriter<AppearEvent>, mut disappear_event: EventWriter<DisappearEvent>) {
    for (observer_entity, mut observers, observer_position, option_monster, option_player) in monsters.iter_mut() {
        let mut new_entities = Vec::<Entity>::new();
        for (player_entity, player_position) in players.iter() {
            if observer_entity != player_entity {
                let entity_type = if let Some(_monster) = option_monster {
                    EntityType::Monster(observer_entity)
                } else if let Some(_player) = option_player {
                    EntityType::Player(observer_entity)
                } else {
                    EntityType::Npc(observer_entity)
                };
                if observer_position.is_in_sight(player_position) {
                    new_entities.push(player_entity);
                    if !observers.entities.contains(&player_entity) {
                        observers.entities.push(player_entity);
                        appear_event.send(AppearEvent { entity: entity_type, observer: player_entity });
                    }
                } else if observers.entities.contains(&player_entity) {
                    disappear_event.send(DisappearEvent { entity: entity_type, observer: player_entity });
                }
            }
        }
        observers.entities = new_entities;
    }
}

fn broadcast_appear(
    mut appear_event: EventReader<AppearEvent>, 
    monsters: Query<(&Id, &Monster, &Position, &CurrentHealthPoints, &MaximumHealthPoints)>, 
    npcs: Query<(&Id, &Npc, &Position, &Direction)>, 
    players: Query<(&Id, &Player, &Position, &Appearance)>,
    observers: Query<&SocketWriter>
) {
    for event in appear_event.read() {
        if let Ok(socket_writer) = observers.get(event.observer) {
            match event.entity {
                EntityType::Player(entity) => {
                    if let Ok((id, player, position, appearence)) = players.get(entity) {
                        let player_appear = PlayerAppearResponse::new(id, player, position, appearence, false);
                        socket_writer.write(&mut (&player_appear).into());
                    }
                },
                EntityType::Monster(entity) => {
                    if let Ok((id, monster, position, current_health_points, maximum_health_points)) = monsters.get(entity) {
                        let monster_appear = MonsterAppearResponse::new(id, monster, position, current_health_points, maximum_health_points);
                        socket_writer.write(&mut (&monster_appear).into());
                    };
                },
                EntityType::Npc(entity) => {
                    if let Ok((id, npc, position, direction)) = npcs.get(entity) {
                        let npc_appear = NpcAppearResponse::new(id, npc, position, direction);
                        socket_writer.write(&mut (&npc_appear).into());
                    };
                },
            };
        }
    }
}

fn broadcast_disappear(
    mut appear_event: EventReader<DisappearEvent>, 
    monsters: Query<&Id, With<Monster>>, 
    players: Query<&Id, With<Player>>, 
    npcs: Query<&Id, With<Npc>>, 
    observers: Query<&SocketWriter>
) {
    for event in appear_event.read() {
        if let Ok(socket_writer) = observers.get(event.observer) {
            match event.entity {
                EntityType::Player(entity) => {
                    if let Ok(id) = players.get(entity) {
                        let player_disappear = PlayerDisappearResponse::new(id);
                        socket_writer.write(&mut (&player_disappear).into());
                    };
                },
                EntityType::Monster(entity) => {
                    if let Ok(id) = monsters.get(entity) {
                        let monster_disappear = MonsterDisappearResponse::new(id);
                        socket_writer.write(&mut (&monster_disappear).into());
                    };
                },
                EntityType::Npc(entity) => {
                    if let Ok(id) = npcs.get(entity) {
                        let monster_disappear = MonsterDisappearResponse::new(id);
                        socket_writer.write(&mut (&monster_disappear).into());
                    };
                },
            }
        }
    }
}