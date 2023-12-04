use crate::framework::packet::Packet;

pub const HEADER: u8 = 34;

#[derive(Debug)]
pub struct PlayerWalkResponse {
    pub player_id: i32,
    pub delta_x: u8,
    pub delta_y: u8,
    pub delta_z: u8,
}

impl From<&mut Packet> for PlayerWalkResponse {
    fn from(packet: &mut Packet) -> Self {
        let player_id = packet.get_i32();
        let delta_x = packet.get_u8();
        let delta_y = packet.get_u8();
        let delta_z = packet.get_u8();
        PlayerWalkResponse { player_id, delta_x, delta_y, delta_z }
    }
}

impl From<&PlayerWalkResponse> for Packet {
    fn from(val: &PlayerWalkResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.player_id);
        packet.write_u8(val.delta_x);
        packet.write_u8(val.delta_y);
        packet.write_u8(val.delta_z);
        packet
    }
}