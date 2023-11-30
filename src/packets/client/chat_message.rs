use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 41;

#[derive(Component, Debug, Clone)]
pub struct ChatMessage {
    pub message: String
}

impl From<&mut Packet> for ChatMessage {
    fn from(packet: &mut Packet) -> Self {
        let message = packet.get_string();
        ChatMessage { message }
    }
}