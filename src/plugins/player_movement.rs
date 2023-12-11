use bevy::prelude::*;
use crate::components::appearance::Appearance;
use crate::components::id::Id;
use crate::components::network_writer::NetworkWriter;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::components::walking::Walking;
use crate::responses::player_stop_walking::PlayerStopWalkingResponse;
use crate::responses::player_walk::PlayerWalkResponse;
use crate::responses::player_appear::PlayerAppearResponse;
use crate::responses::player_position::PlayerPositionResponse;
use crate::requests::player_stop_walking::PlayerStopWalkingRequest;
use crate::requests::player_walk::PlayerWalkRequest;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_player_walking);
        app.add_systems(Update, handle_position_added);
        app.add_systems(Update, handle_player_walk);
        app.add_systems(Update, handle_player_stop_walking);
        app.add_systems(Last, update_previous_position);
    }
}

fn handle_position_added(query: Query<(&Id, &Player, &Position, &Appearance, &NetworkWriter), Added<Position>>) {
    for (id, player, position, appearence, socket_writer) in &query {
        let player_position = PlayerPositionResponse { unknown: vec![47, 1], x: position.x, y: position.y };
        socket_writer.write(&mut (&player_position).into());
        let player_appear = PlayerAppearResponse::new(id, player, position, appearence, true);
        socket_writer.write(&mut (&player_appear).into());
    }
}

fn handle_player_walking(mut commands: Commands, moved_query: Query<(Entity, &Id, &Position, &Walking), With<Player>>, players_query: Query<(&Id, &Position, &NetworkWriter), With<Player>>) {
    for (entity, walking_id, walking_position, walking) in &moved_query {
        for (id, position, socket_writer) in &players_query {
            if walking_id.id != id.id && position.is_in_sight(walking_position) {
                if walking.done {
                    let player_walk = PlayerStopWalkingResponse { 
                        player_id: walking_id.id, 
                        delta_x: walking.delta_x, 
                        delta_y: walking.delta_y, 
                        delta_z: walking.delta_z 
                    };
                    socket_writer.write(&mut (&player_walk).into());
                } else {
                    let player_walk = PlayerWalkResponse { 
                        player_id: walking_id.id, 
                        delta_x: walking.delta_x, 
                        delta_y: walking.delta_y, 
                        delta_z: walking.delta_z 
                    };
                    socket_writer.write(&mut (&player_walk).into());
                }
            }
        }
        commands.entity(entity).remove::<Walking>();
    }
}

fn update_position(position: &mut Position, delta_x: u8, delta_y: u8, delta_z: u8) {
    let delta_x: u32 = delta_x.try_into().unwrap();
    if delta_x > 128 { 
        position.x -= 256 - delta_x;
    } else { 
        position.x += delta_x;
    }
    let delta_y: u32 = delta_y.try_into().unwrap();
    if delta_y > 128 { 
        position.y -= 256 - delta_y;
    } else { 
        position.y += delta_y;
    }
    let delta_z: u32 = delta_z.try_into().unwrap();
    if delta_z > 128 { 
        position.z -= 256 - delta_z;
    } else { 
        position.z += delta_z;
    }
}

fn handle_player_walk(mut commands: Commands, mut query: Query<(Entity, &PlayerWalkRequest, &mut Position)>) {
    for (entity, client_packet, mut position) in query.iter_mut() {
        update_position(&mut position, client_packet.delta_x, client_packet.delta_y, client_packet.delta_z);
        commands.entity(entity).insert(Walking { done: false, delta_x: client_packet.delta_x, delta_y: client_packet.delta_y, delta_z: client_packet.delta_z });
        commands.entity(entity).remove::<PlayerWalkRequest>();
    }
}

fn handle_player_stop_walking(mut commands: Commands, mut query: Query<(Entity, &PlayerStopWalkingRequest, &mut Position)>) {
    for (entity, client_packet, mut position) in query.iter_mut() {
        update_position(&mut position, client_packet.delta_x, client_packet.delta_y, client_packet.delta_z);
        commands.entity(entity).insert(Walking { done: true, delta_x: client_packet.delta_x, delta_y: client_packet.delta_y, delta_z: client_packet.delta_z });
        commands.entity(entity).remove::<PlayerStopWalkingRequest>();
    }
}

fn update_previous_position(mut query: Query<(&mut Previous<Position>, &Position)>) {
    for (mut previous_position, position) in query.iter_mut() {
        previous_position.entity.x = position.x;
        previous_position.entity.y = position.y;
        previous_position.entity.z = position.z;
    }
}