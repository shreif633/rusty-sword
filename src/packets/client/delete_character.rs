use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 2;

#[derive(Component, Debug, Clone)]
pub struct DeleteCharacter {
    pub character_id: u32,
}

impl From<&mut Packet> for DeleteCharacter {
    fn from(packet: &mut Packet) -> Self {
        let character_id = packet.get_u32();
        DeleteCharacter { character_id }
    }
}