use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 57;

#[derive(Component, Debug, Clone)]
pub struct UnequipItem {
    pub item_id: u32,
}

impl From<&mut Packet> for UnequipItem {
    fn from(packet: &mut Packet) -> Self {
        let item_id = packet.get_u32();
        UnequipItem { item_id }
    }
}