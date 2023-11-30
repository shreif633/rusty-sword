use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 6;

#[derive(Component, Debug, Clone)]
pub struct RestoreDeletedCharacter {
    pub character_id: u32,
}

impl From<&mut Packet> for RestoreDeletedCharacter {
    fn from(packet: &mut Packet) -> Self {
        let character_id = packet.get_u32();
        RestoreDeletedCharacter { character_id }
    }
}