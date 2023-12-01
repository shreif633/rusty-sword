use crate::framework::packet::Packet;

pub const HEADER: u8 = 42;

#[derive(Debug)]
pub struct ServerSelectedResponse {
    pub unknown: Vec<u8>
}

impl ServerSelectedResponse {
    pub fn new() -> Self {
        ServerSelectedResponse { unknown: vec![242, 108, 141, 16, 54, 212, 76, 126, 68, 30, 207, 86, 101, 97, 30, 0, 0, 118, 0, 0, 0, 0, 0, 0, 0, 2, 18, 2] }
    }
}

impl From<&mut Packet> for ServerSelectedResponse {
    fn from(packet: &mut Packet) -> Self {
        ServerSelectedResponse { unknown: packet.get_buffer(28) }
    }
}

impl From<&ServerSelectedResponse> for Packet {
    fn from(val: &ServerSelectedResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_buffer(&val.unknown);
        packet
    }
}