use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 57;

#[derive(Component, Debug, Clone)]
pub struct UnequipItemRequest {
    pub item_id: i32,
}

impl From<&mut Packet> for UnequipItemRequest {
    fn from(packet: &mut Packet) -> Self {
        let item_id = packet.get_i32();
        UnequipItemRequest { item_id }
    }
}