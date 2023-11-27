use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 46;

#[derive(Component, Debug, Clone)]
pub struct Emote {
    pub unsafe_player_id: u32,
    pub emote_index: u8
}

impl From<&mut Packet> for Emote {
    fn from(packet: &mut Packet) -> Self {
        let unsafe_player_id = packet.get_u32();
        let emote_index = packet.get_u8();
        Emote { unsafe_player_id, emote_index }
    }
}