use crate::framework::packet::Packet;

pub const HEADER: u8 = 97;

#[derive(Debug)]
pub struct ServerSelect {
    pub unknown: Vec<u8>
}

impl From<&mut Packet> for ServerSelect {
    fn from(packet: &mut Packet) -> Self {
        ServerSelect { unknown: packet.get_buffer(21) }
    }
}