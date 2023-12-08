use bevy::prelude::*;
use crate::enums::target_type::TargetType;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 16;

#[derive(Component, Debug, Clone)]
pub struct NormalHitRequest {
    pub target_type: TargetType,
    pub target_id: i32,
    pub unknown: u32
}

impl From<&mut Packet> for NormalHitRequest {
    fn from(packet: &mut Packet) -> Self {
        let target_type = TargetType::from(packet.get_u8());
        let target_id = packet.get_i32();
        let unknown = packet.get_u32();
        NormalHitRequest { target_type, target_id, unknown }
    }
}