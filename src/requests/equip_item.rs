use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 100;

#[derive(Component, Debug, Clone)]
pub struct EquipItemRequest {
    pub item_id: u32,
}

impl From<&mut Packet> for EquipItemRequest {
    fn from(packet: &mut Packet) -> Self {
        let item_id = packet.get_u32();
        EquipItemRequest { item_id }
    }
}