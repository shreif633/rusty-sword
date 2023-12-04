use bevy::prelude::*;
use crate::components::direction::Direction;
use crate::components::npc::NPC;
use crate::components::position::Position;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 52;

#[derive(Debug)]
pub struct NPCAppearResponse {
    pub npc_id: u32,
    pub npc_index: u16,
    pub npc_shape: u8,
    pub npc_x: u32,
    pub npc_y: u32,
    pub npc_z: u32,
    pub npc_direction: u16,
    pub general_state: i64,
    pub flag: u32,
}

impl NPCAppearResponse {
    pub fn new(entity: Entity, npc: &NPC, position: &Position, direction: &Direction) -> Self {
        NPCAppearResponse {
            npc_id: entity.index(),
            npc_index: npc.index,
            npc_shape: npc.shape,
            npc_x: position.x,
            npc_y: position.y,
            npc_z: position.z,
            npc_direction: direction.direction,
            general_state: 0,
            flag: 0,
        }
    }
}

impl From<&mut Packet> for NPCAppearResponse {
    fn from(packet: &mut Packet) -> Self {
        let npc_id = packet.get_u32();
        let npc_index = packet.get_u16();
        let npc_shape = packet.get_u8();
        let npc_x = packet.get_u32();
        let npc_y = packet.get_u32();
        let npc_z = packet.get_u32();
        if npc_index == 205 {
            println!("pkt 205: {:?}", packet);
        }
        let npc_direction = packet.get_u16();
        let general_state = packet.get_i64();
        let flag = packet.get_u32();
        NPCAppearResponse { npc_id, npc_index, npc_shape, npc_x, npc_y, npc_z, npc_direction, general_state, flag }
    }
}

impl From<&NPCAppearResponse> for Packet {
    fn from(val: &NPCAppearResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(val.npc_id);
        packet.write_u16(val.npc_index);
        packet.write_u8(val.npc_shape);
        packet.write_u32(val.npc_x);
        packet.write_u32(val.npc_y);
        packet.write_u32(val.npc_z);
        packet.write_u16(val.npc_direction);
        packet.write_i64(val.general_state);
        packet.write_u32(val.flag);
        packet
    }
}