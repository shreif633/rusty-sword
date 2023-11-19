use crate::framework::packet::Packet;

pub const HEADER: u8 = 4;

#[derive(Debug)]
pub struct SelectCharacter {
    pub character_id: u32,
    pub unknown: Vec<u8>,
}

impl From<&mut Packet> for SelectCharacter {
    fn from(packet: &mut Packet) -> Self {
        let character_id = packet.get_u32();
        let unknown = packet.get_buffer(8);
        SelectCharacter { character_id, unknown }
    }
}