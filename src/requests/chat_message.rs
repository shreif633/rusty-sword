use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 41;

#[derive(Component, Debug, Clone)]
pub struct ChatMessageRequest {
    pub message: String
}

impl From<&mut Packet> for ChatMessageRequest {
    fn from(packet: &mut Packet) -> Self {
        let message = packet.get_string();
        ChatMessageRequest { message }
    }
}