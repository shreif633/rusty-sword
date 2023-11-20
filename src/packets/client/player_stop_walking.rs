use crate::framework::packet::Packet;

pub const HEADER: u8 = 146;

#[derive(Debug)]
pub struct PlayerStopWalking {
    pub delta_x: u8,
    pub delta_y: u8,
    pub delta_z: u8,
}

impl From<&mut Packet> for PlayerStopWalking {
    fn from(packet: &mut Packet) -> Self {
        let delta_x = packet.get_u8();
        let delta_y = packet.get_u8();
        let delta_z = packet.get_u8();
        PlayerStopWalking { delta_x, delta_y, delta_z }
    }
}