use crate::framework::packet::Packet;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 7;

#[derive(Debug)]
pub struct PlayerCurrentHealthPointsResponse {
    pub current_health_points: u32,
}

impl From<&mut Packet> for PlayerCurrentHealthPointsResponse {
    fn from(packet: &mut Packet) -> Self {
        let current_health_points = packet.get_u32();
        PlayerCurrentHealthPointsResponse { current_health_points }
    }
}

impl From<&PlayerCurrentHealthPointsResponse> for Packet {
    fn from(val: &PlayerCurrentHealthPointsResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_u32(val.current_health_points);
        packet
    }
}