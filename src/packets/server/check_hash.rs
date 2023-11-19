use crate::framework::packet::Packet;

pub const HEADER: u8 = 255;
pub const SUB_HEADER: u32 = 254;

#[derive(Debug)]
pub struct CheckHash {
    pub hash: u32,
}

impl From<&mut Packet> for CheckHash {
    fn from(packet: &mut Packet) -> Self {
        let hash = packet.get_u32();
        CheckHash { hash }
    }
}

impl From<&CheckHash> for Packet {
    fn from(val: &CheckHash) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(SUB_HEADER);
        packet.write_u32(val.hash);
        packet
    }
}