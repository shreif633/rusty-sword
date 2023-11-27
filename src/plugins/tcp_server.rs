use bevy::prelude::*;
use std::sync::{Mutex, Arc};
use tokio::sync::mpsc::Sender;

use crate::{packets::client::ClientPacket, framework::packet::Packet};

pub struct TcpServerPlugin;

impl Plugin for TcpServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, process_socket_queue);
    }
}

#[derive(Component)]
pub struct UserAddr {
    pub socket_addr: String
}

#[derive(Resource, Debug)]
pub struct SocketQueue {
    pub queue: Arc<Mutex<Vec<SocketPair>>>
}

#[derive(Debug)]
pub struct SocketPair(pub String, pub SocketMessage);

#[derive(Debug)]
pub enum SocketMessage {
    Connected(Sender<Vec<u8>>),
    Packet(ClientPacket),
    Disconnected,
}

#[derive(Debug, Component)]
pub struct SocketWriter {
    pub socket_writer: Sender<Vec<u8>>
}

impl SocketWriter {
    pub fn write(&self, packet: &mut Packet) {
        let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
        let buffer = packet.serialize();
        rt.block_on(async move {
            self.socket_writer.send(buffer).await.unwrap();
        });
    }
}

fn process_socket_queue(mut commands: Commands, queue: ResMut<SocketQueue>, entities: Query<(Entity, &UserAddr)>) {
    let mut queue = queue.queue.lock().unwrap();
    queue.drain(0..).into_iter().for_each(|connection_pair| {
        match connection_pair.1 {
            SocketMessage::Connected(socket_writer) => {
                commands.spawn((SocketWriter { socket_writer }, UserAddr { socket_addr: connection_pair.0.clone() }));
            },
            SocketMessage::Packet(client_packet) => {
                for (entity, user_addr) in &entities {
                    if user_addr.socket_addr == connection_pair.0 {
                        match client_packet {
                            ClientPacket::ServerSelect(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::Authenticate(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::DeleteCharacter(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::RestoreDeletedCharacter(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::CreateCharacter(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::SelectCharacter(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            // ClientPacket::SkillPrepare(_) => todo!(),
                            ClientPacket::ChatMessage(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::PlayerWalk(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::PlayerStopWalking(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::Emote(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            _ => println!("{:?}", client_packet),
                        };
                    }
                }
            },
            SocketMessage::Disconnected => (),
        };
    })
}