use crate::framework::packet::Packet;

pub const HEADER: u8 = 255;
pub const SUB_HEADER: u32 = 254;

#[derive(Debug)]
pub struct CheckHashResponse {
    pub hash: u32,
}

impl CheckHashResponse {
    pub fn new() -> Self {
        CheckHashResponse { hash: 1325039837 }
    }
}

impl From<&mut Packet> for CheckHashResponse {
    fn from(packet: &mut Packet) -> Self {
        let hash = packet.get_u32();
        CheckHashResponse { hash }
    }
}

impl From<&CheckHashResponse> for Packet {
    fn from(val: &CheckHashResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(SUB_HEADER);
        packet.write_u32(val.hash);
        packet
    }
}