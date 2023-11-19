use crate::framework::packet::Packet;

pub const HEADER: u8 = 42;

#[derive(Debug)]
pub struct ServerSelected {
    pub unknown: Vec<u8>
}

impl From<&mut Packet> for ServerSelected {
    fn from(packet: &mut Packet) -> Self {
        ServerSelected { unknown: packet.get_buffer(28) }
    }
}

impl From<&ServerSelected> for Packet {
    fn from(val: &ServerSelected) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_buffer(&val.unknown);
        packet
    }
}