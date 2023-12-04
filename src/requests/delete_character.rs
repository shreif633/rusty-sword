use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 2;

#[derive(Component, Debug, Clone)]
pub struct DeleteCharacterRequest {
    pub character_id: i32,
}

impl From<&mut Packet> for DeleteCharacterRequest {
    fn from(packet: &mut Packet) -> Self {
        let character_id = packet.get_i32();
        DeleteCharacterRequest { character_id }
    }
}