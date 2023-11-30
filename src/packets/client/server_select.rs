use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 97;

#[derive(Component, Debug, Clone)]
pub struct ServerSelect {
    pub unknown: Vec<u8>
}

impl From<&mut Packet> for ServerSelect {
    fn from(packet: &mut Packet) -> Self {
        ServerSelect { unknown: packet.get_buffer(21) }
    }
}