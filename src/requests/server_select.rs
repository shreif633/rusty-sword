use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 97;

#[derive(Component, Debug, Clone)]
pub struct ServerSelectRequest {
    pub unknown: Vec<u8>
}

impl From<&mut Packet> for ServerSelectRequest {
    fn from(packet: &mut Packet) -> Self {
        ServerSelectRequest { unknown: packet.get_buffer(21) }
    }
}