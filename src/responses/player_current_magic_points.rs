use crate::framework::packet::Packet;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 8;

#[derive(Debug)]
pub struct PlayerCurrentMagicPointsResponse {
    pub current_magic_points: u16,
}

impl From<&mut Packet> for PlayerCurrentMagicPointsResponse {
    fn from(packet: &mut Packet) -> Self {
        let current_magic_points = packet.get_u16();
        PlayerCurrentMagicPointsResponse { current_magic_points }
    }
}

impl From<&PlayerCurrentMagicPointsResponse> for Packet {
    fn from(val: &PlayerCurrentMagicPointsResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_u16(val.current_magic_points);
        packet
    }
}