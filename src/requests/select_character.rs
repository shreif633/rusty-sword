use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 4;

#[derive(Component, Debug, Clone)]
pub struct SelectCharacterRequest {
    pub character_id: u32,
    pub unknown: Vec<u8>,
}

impl From<&mut Packet> for SelectCharacterRequest {
    fn from(packet: &mut Packet) -> Self {
        let character_id = packet.get_u32();
        let unknown = packet.get_buffer(8);
        SelectCharacterRequest { character_id, unknown }
    }
}