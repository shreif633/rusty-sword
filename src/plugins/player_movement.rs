use bevy::prelude::*;
use crate::components::appearence::Appearence;
use crate::components::job::Job;
use crate::components::player::Player;
use crate::components::position::{Coordinate, Position};
use crate::components::walking::Walking;
use crate::responses::player_stop_walking::PlayerStopWalkingResponse;
use crate::responses::player_walk::PlayerWalkResponse;
use crate::responses::player_appear::PlayerAppearResponse;
use crate::responses::player_position::PlayerPositionResponse;
use crate::requests::player_stop_walking::PlayerStopWalkingRequest;
use crate::requests::player_walk::PlayerWalkRequest;
use super::tcp_server::SocketWriter;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_player_walking);
        app.add_systems(Update, handle_position_added);
        app.add_systems(Update, handle_position_change);
        app.add_systems(Update, handle_player_walk);
        app.add_systems(Update, handle_player_stop_walking);
    }
}

#[derive(Component)]
pub struct PreviousPosition {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Coordinate for &PreviousPosition {
    fn get_xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

fn handle_position_added(query: Query<(Added<Position>, &Player, &Job, &Position, &Appearence, &SocketWriter)>) {
    for (added_position, player, job, position, appearence, socket_writer) in &query {
        if added_position {
            let player_position = PlayerPositionResponse { unknown: vec![47, 1], x: position.x, y: position.y };
            socket_writer.write(&mut (&player_position).into());
            let player_appear = PlayerAppearResponse::new(&player, &job, &position, &appearence, true);
            socket_writer.write(&mut (&player_appear).into());
        }
    }
}

fn handle_position_change(moved_query: Query<(&Player, Changed<Position>, &Job, &PreviousPosition, &Position, &Appearence, &SocketWriter)>, players_query: Query<(&Player, &Job, &Position, &Appearence, &SocketWriter)>) {
    for (moved_player, moved_position_changed, moved_job, moved_previous_position, moved_position, moved_appearence, moved_socket_writer) in &moved_query {
        if moved_position_changed {
            for (player, job, position, appearence, socket_writer) in &players_query {
                if moved_player.id != player.id {
                    if !position.is_in_sight(&moved_previous_position) {
                        if position.is_in_sight(&moved_position) {
                            let player_appear = PlayerAppearResponse::new(&moved_player, &moved_job, &moved_position, &moved_appearence, false);
                            socket_writer.write(&mut (&player_appear).into());
                            let player_appear = PlayerAppearResponse::new(&player, &job, &position, &appearence, false);
                            moved_socket_writer.write(&mut (&player_appear).into());
                        }
                    }
                }
            }
        }
    }
}

fn handle_player_walking(mut commands: Commands, moved_query: Query<(Entity, &Player, &Position, &Walking)>, players_query: Query<(&Player, &Position, &SocketWriter)>) {
    for (entity, walking_player, walking_position, walking) in &moved_query {
        for (player, position, socket_writer) in &players_query {
            if walking_player.id != player.id {
                if position.is_in_sight(&walking_position) {
                    if walking.done {
                        let player_walk = PlayerStopWalkingResponse { 
                            player_id: walking_player.id, 
                            delta_x: walking.delta_x, 
                            delta_y: walking.delta_y, 
                            delta_z: walking.delta_z 
                        };
                        socket_writer.write(&mut (&player_walk).into());
                    } else {
                        let player_walk = PlayerWalkResponse { 
                            player_id: walking_player.id, 
                            delta_x: walking.delta_x, 
                            delta_y: walking.delta_y, 
                            delta_z: walking.delta_z 
                        };
                        socket_writer.write(&mut (&player_walk).into());
                    }
                }
            }
        }
        commands.entity(entity).remove::<Walking>();
    }
}

fn update_position(previous_position: &mut PreviousPosition, position: &mut Position, delta_x: u8, delta_y: u8, delta_z: u8) {
    previous_position.x = position.x;
    previous_position.y = position.y;
    previous_position.z = position.z;
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

fn handle_player_walk(mut commands: Commands, mut query: Query<(Entity, &PlayerWalkRequest, &mut PreviousPosition, &mut Position)>) {
    for (entity, client_packet, mut previous_position, mut position) in query.iter_mut() {
        update_position(&mut previous_position, &mut position, client_packet.delta_x, client_packet.delta_y, client_packet.delta_z);
        commands.entity(entity).insert(Walking { done: false, delta_x: client_packet.delta_x, delta_y: client_packet.delta_y, delta_z: client_packet.delta_z });
        commands.entity(entity).remove::<PlayerWalkRequest>();
    }
}

fn handle_player_stop_walking(mut commands: Commands, mut query: Query<(Entity, &PlayerStopWalkingRequest, &mut PreviousPosition, &mut Position)>) {
    for (entity, client_packet, mut previous_position, mut position) in query.iter_mut() {
        update_position(&mut previous_position, &mut position, client_packet.delta_x, client_packet.delta_y, client_packet.delta_z);
        commands.entity(entity).insert(Walking { done: true, delta_x: client_packet.delta_x, delta_y: client_packet.delta_y, delta_z: client_packet.delta_z });
        commands.entity(entity).remove::<PlayerStopWalkingRequest>();
    }
}