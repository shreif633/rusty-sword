use bevy::prelude::*;
use std::sync::{Mutex, Arc};
use tokio::sync::mpsc::Sender;
use crate::framework::packet::Packet;
use crate::requests::ClientPacket;

pub struct TcpServerPlugin;

impl Plugin for TcpServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, process_socket_queue);
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
    queue.drain(0..).for_each(|connection_pair| {
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
                            ClientPacket::ChatMessage(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::PlayerWalk(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::PlayerStopWalking(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::Emote(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::EquipItem(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::UnequipItem(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::UseItem(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            _ => {
                                if let ClientPacket::Unknown(ref client_packet) = client_packet {
                                    if *client_packet.buffer.get(2).unwrap() != 253 {
                                        println!("{:?}", client_packet)
                                    }
                                } else {
                                    println!("{:?}", client_packet)
                                }
                            },
                        };
                    }
                }
            },
            SocketMessage::Disconnected => (),
        };
    })
}