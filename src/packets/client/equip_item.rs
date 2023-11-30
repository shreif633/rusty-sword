use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 100;

#[derive(Component, Debug, Clone)]
pub struct EquipItem {
    pub item_id: u32,
}

impl From<&mut Packet> for EquipItem {
    fn from(packet: &mut Packet) -> Self {
        let item_id = packet.get_u32();
        EquipItem { item_id }
    }
}