use crate::framework::packet::Packet;

pub const HEADER: u8 = 35;

#[derive(Debug)]
pub struct PlayerStopWalkingResponse {
    pub player_id: u32,
    pub delta_x: u8,
    pub delta_y: u8,
    pub delta_z: u8,
}

impl From<&mut Packet> for PlayerStopWalkingResponse {
    fn from(packet: &mut Packet) -> Self {
        let player_id = packet.get_u32();
        let delta_x = packet.get_u8();
        let delta_y = packet.get_u8();
        let delta_z = packet.get_u8();
        PlayerStopWalkingResponse { player_id, delta_x, delta_y, delta_z }
    }
}

impl From<&PlayerStopWalkingResponse> for Packet {
    fn from(val: &PlayerStopWalkingResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(val.player_id);
        packet.write_u8(val.delta_x);
        packet.write_u8(val.delta_y);
        packet.write_u8(val.delta_z);
        packet
    }
}