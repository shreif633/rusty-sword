use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 64;

#[derive(Component, Debug, Clone)]
pub struct UseItemRequest {
    pub item_id: i32,
}

impl From<&mut Packet> for UseItemRequest {
    fn from(packet: &mut Packet) -> Self {
        let item_id = packet.get_i32();
        UseItemRequest { item_id }
    }
}