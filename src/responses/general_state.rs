use crate::framework::packet::Packet;

pub const HEADER: u8 = 148;

#[derive(Debug)]
pub struct GeneralStateResponse {
    pub target_id: i32,
    pub general_state: i64,
    pub speed: Option<u32>,
}

impl From<&mut Packet> for GeneralStateResponse {
    fn from(packet: &mut Packet) -> Self {
        let target_id = packet.get_i32();
        let general_state = packet.get_i64();
        let speed = if packet.size() > 15 {
            Some(packet.get_u32())
        } else {
            None
        };
        GeneralStateResponse { target_id, general_state, speed }
    }
}

impl From<&GeneralStateResponse> for Packet {
    fn from(val: &GeneralStateResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.target_id);
        packet.write_i64(val.general_state);
        if let Some(speed) = val.speed {
            packet.write_u32(speed);
        }
        packet
    }
}