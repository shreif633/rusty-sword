use bevy::prelude::*;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 177;

#[derive(Component, Debug, Clone)]
pub struct SelectTargetRequest {
    pub target_id: i32,
}

impl From<&mut Packet> for SelectTargetRequest {
    fn from(packet: &mut Packet) -> Self {
        let target_id = packet.get_i32();
        SelectTargetRequest { target_id }
    }
}