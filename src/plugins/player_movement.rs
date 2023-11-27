use bevy::prelude::*;
use crate::packets::{client::{player_walk::PlayerWalk, player_stop_walking::PlayerStopWalking}, server::{player_position::PlayerPosition, player_appear::PlayerAppear}};
use super::{tcp_server::SocketWriter, select_character::{Player, Appearence}};

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
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Position {
    pub fn calculate_distance<T: Coordinate>(&self, b: &T) -> u32 {
        let (bx, by) = b.get_xy();
        let x_diff = self.x as f64 - bx as f64;
        let y_diff = self.y as f64 - by as f64;
        // Euclidean distance formula: sqrt((x2 - x1)^2 + (y2 - y1)^2)
        ((x_diff.powi(2) + y_diff.powi(2)) as f64).sqrt().round() as u32
    }

    pub fn is_in_range<T: Coordinate>(&self, b: &T, range: u32) -> bool {
        self.calculate_distance(b) < range
    }

    pub fn is_in_sight<T: Coordinate>(&self, b: &T) -> bool {
        self.is_in_range(b, 900)
    }
}

impl Coordinate for &Position {
    fn get_xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

pub trait Coordinate {
    fn get_xy(&self) -> (u32, u32);
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

#[derive(Component, Debug)]
pub struct Walking {
    done: bool,
    delta_x: u8,
    delta_y: u8,
    delta_z: u8,
}

fn build_player_appear_packet(player: &Player, position: &Position, appearence: &Appearence, is_current_player: bool) -> PlayerAppear {
    PlayerAppear { 
        player_id: player.id, 
        name: appearence.name.clone(), 
        class: match player.class {
            0 => crate::packets::server::player_appear::PlayerClass::Knight,
            1 => crate::packets::server::player_appear::PlayerClass::Mage,
            _ => crate::packets::server::player_appear::PlayerClass::Archer,
        }, 
        is_current_player,
        x: position.x, 
        y: position.y, 
        z: position.z, 
        unknown1: vec![1, 0, 0, 0, 0, 136, 0, 0, 0, 0], 
        weapon_index: appearence.weapon_index, 
        shield_index: appearence.shield_index, 
        helmet_index: appearence.helmet_index, 
        chest_index: appearence.chest_index, 
        shorts_index: appearence.shorts_index, 
        gloves_index: appearence.gloves_index, 
        boots_index: appearence.boots_index, 
        unknown2: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
        face: appearence.face, 
        hair: appearence.hair, 
        unknown3: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 2, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] 
    }
}

fn handle_position_added(query: Query<(Added<Position>, &Player, &Position, &Appearence, &SocketWriter)>) {
    for (added_position, player, position, appearence, socket_writer) in &query {
        if added_position {
            let player_position = PlayerPosition { unknown: vec![47, 1], x: position.x, y: position.y };
            socket_writer.write(&mut (&player_position).into());
            let player_appear = build_player_appear_packet(&player, &position, &appearence, true);
            socket_writer.write(&mut (&player_appear).into());
        }
    }
}

fn handle_position_change(moved_query: Query<(&Player, Changed<Position>, &PreviousPosition, &Position, &Appearence, &SocketWriter)>, players_query: Query<(&Player, &Position, &Appearence, &SocketWriter)>) {
    for (moved_player, moved_position_changed, moved_previous_position, moved_position, moved_appearence, moved_socket_writer) in &moved_query {
        if moved_position_changed {
            for (player, position, appearence, socket_writer) in &players_query {
                if moved_player.id != player.id {
                    if !position.is_in_sight(&moved_previous_position) {
                        if position.is_in_sight(&moved_position) {
                            let player_appear = build_player_appear_packet(&moved_player, &moved_position, &moved_appearence, false);
                            socket_writer.write(&mut (&player_appear).into());
                            let player_appear = build_player_appear_packet(&player, &position, &appearence, false);
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
                        let player_walk = crate::packets::server::player_stop_walking::PlayerStopWalking { 
                            player_id: walking_player.id, 
                            delta_x: walking.delta_x, 
                            delta_y: walking.delta_y, 
                            delta_z: walking.delta_z 
                        };
                        socket_writer.write(&mut (&player_walk).into());
                    } else {
                        let player_walk = crate::packets::server::player_walk::PlayerWalk { 
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

fn handle_player_walk(mut commands: Commands, mut query: Query<(Entity, &PlayerWalk, &mut PreviousPosition, &mut Position)>) {
    for (entity, client_packet, mut previous_position, mut position) in query.iter_mut() {
        update_position(&mut previous_position, &mut position, client_packet.delta_x, client_packet.delta_y, client_packet.delta_z);
        commands.entity(entity).insert(Walking { done: false, delta_x: client_packet.delta_x, delta_y: client_packet.delta_y, delta_z: client_packet.delta_z });
        commands.entity(entity).remove::<PlayerWalk>();
    }
}

fn handle_player_stop_walking(mut commands: Commands, mut query: Query<(Entity, &PlayerStopWalking, &mut PreviousPosition, &mut Position)>) {
    for (entity, client_packet, mut previous_position, mut position) in query.iter_mut() {
        update_position(&mut previous_position, &mut position, client_packet.delta_x, client_packet.delta_y, client_packet.delta_z);
        commands.entity(entity).insert(Walking { done: true, delta_x: client_packet.delta_x, delta_y: client_packet.delta_y, delta_z: client_packet.delta_z });
        commands.entity(entity).remove::<PlayerStopWalking>();
    }
}