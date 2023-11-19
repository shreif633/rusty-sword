use crate::framework::packet::Packet;

pub const HEADER: u8 = 27;

#[derive(Debug)]
pub struct PlayerPosition {
    pub unknown: Vec<u8>,
    pub x: u32,
    pub y: u32,
}

impl From<&mut Packet> for PlayerPosition {
    fn from(packet: &mut Packet) -> Self {
        let unknown = packet.get_buffer(2);
        let x = packet.get_u32();
        let y = packet.get_u32();
        PlayerPosition { unknown, x, y  }
    }
}

impl From<&PlayerPosition> for Packet {
    fn from(val: &PlayerPosition) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_buffer(&val.unknown);
        packet.write_u32(val.x);
        packet.write_u32(val.y);
        packet
    }
}