use crate::components::direction::Direction;
use crate::components::id::Id;
use crate::components::npc::Npc;
use crate::components::position::Position;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 52;

#[derive(Debug)]
pub struct NpcAppearResponse {
    pub id: i32,
    pub index: u16,
    pub shape: u8,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub direction: u16,
    pub general_state: i64,
    pub flag: u32,
}

impl NpcAppearResponse {
    pub fn new(id: &Id, npc: &Npc, position: &Position, direction: &Direction) -> Self {
        NpcAppearResponse {
            id: id.id,
            index: npc.index,
            shape: npc.shape,
            x: position.x,
            y: position.y,
            z: position.z,
            direction: direction.direction,
            general_state: 0,
            flag: 0,
        }
    }
}

impl From<&mut Packet> for NpcAppearResponse {
    fn from(packet: &mut Packet) -> Self {
        let id = packet.get_i32();
        let index = packet.get_u16();
        let shape = packet.get_u8();
        let x = packet.get_u32();
        let y = packet.get_u32();
        let z = packet.get_u32();
        if index == 205 {
            println!("pkt 205: {:?}", packet);
        }
        let direction = packet.get_u16();
        let general_state = packet.get_i64();
        let flag = packet.get_u32();
        NpcAppearResponse { id, index, shape, x, y, z, direction, general_state, flag }
    }
}

impl From<&NpcAppearResponse> for Packet {
    fn from(val: &NpcAppearResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.id);
        packet.write_u16(val.index);
        packet.write_u8(val.shape);
        packet.write_u32(val.x);
        packet.write_u32(val.y);
        packet.write_u32(val.z);
        packet.write_u16(val.direction);
        packet.write_i64(val.general_state);
        packet.write_u32(val.flag);
        packet
    }
}