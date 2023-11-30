use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 64;

#[derive(Component, Debug, Clone)]
pub struct UseItem {
    pub item_id: u32,
}

impl From<&mut Packet> for UseItem {
    fn from(packet: &mut Packet) -> Self {
        let item_id = packet.get_u32();
        UseItem { item_id }
    }
}