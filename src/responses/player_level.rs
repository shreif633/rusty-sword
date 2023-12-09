use crate::framework::packet::Packet;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 26;

#[derive(Debug)]
pub struct PlayerLevelResponse {
    pub level: u8,
}

impl From<&mut Packet> for PlayerLevelResponse {
    fn from(packet: &mut Packet) -> Self {
        let level = packet.get_u8();
        PlayerLevelResponse { level  }
    }
}

impl From<&PlayerLevelResponse> for Packet {
    fn from(val: &PlayerLevelResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_u8(val.level);
        packet
    }
}