use crate::framework::packet::Packet;

pub const HEADER: u8 = 254;

#[derive(Debug)]
pub struct Analyze {
    pub unknown: Vec<u8>
}

impl From<&mut Packet> for Analyze {
    fn from(packet: &mut Packet) -> Self {
        Analyze { unknown: packet.buffer.clone() }
    }
}

impl From<&Analyze> for Packet {
    fn from(val: &Analyze) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_buffer(&val.unknown);
        packet
    }
}