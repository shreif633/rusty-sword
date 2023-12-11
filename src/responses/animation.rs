use crate::framework::packet::Packet;

pub const HEADER: u8 = 61;

#[derive(Debug)]
pub struct AnimationResponse {
    pub player_id: i32,
    pub animation_index: u8,
    pub skill_index: Option<u8>,
    pub target_id: Option<i32>,
}

impl From<&mut Packet> for AnimationResponse {
    fn from(packet: &mut Packet) -> Self {
        let player_id = packet.get_i32();
        let unknown = packet.get_u8();
        let (skill_index, target_id) = if packet.size() == 8 {
            (None, None)
        } else {
            let skill_index = packet.get_u8();
            let target_id = packet.get_i32();
            (Some(skill_index), Some(target_id))
        };
        AnimationResponse { player_id, animation_index: unknown, skill_index, target_id }
    }
}

impl From<&AnimationResponse> for Packet {
    fn from(val: &AnimationResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.player_id);
        packet.write_u8(val.animation_index);
        if let Some(skill_index) = val.skill_index {
            packet.write_u8(skill_index);
        }
        if let Some(target_id) = val.target_id {
            packet.write_i32(target_id);
        }
        packet
    }
}