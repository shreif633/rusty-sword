use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 6;

#[derive(Component, Debug, Clone)]
pub struct RestoreDeletedCharacterRequest {
    pub character_id: i32,
}

impl From<&mut Packet> for RestoreDeletedCharacterRequest {
    fn from(packet: &mut Packet) -> Self {
        let character_id = packet.get_i32();
        RestoreDeletedCharacterRequest { character_id }
    }
}