use crate::framework::packet::Packet;

pub const HEADER: u8 = 61;

#[derive(Debug)]
pub struct SkillPrepareResponse {
    pub player_id: i32,
    pub unknown: u8,
    pub skill_index: Option<u8>,
    pub target_id: Option<u32>,
}

impl From<&mut Packet> for SkillPrepareResponse {
    fn from(packet: &mut Packet) -> Self {
        let player_id = packet.get_i32();
        let unknown = packet.get_u8();
        let (skill_index, target_id) = if packet.size() == 8 {
            (None, None)
        } else {
            let skill_index = packet.get_u8();
            let target_id = packet.get_u32();
            (Some(skill_index), Some(target_id))
        };
        SkillPrepareResponse { player_id, unknown, skill_index, target_id }
    }
}

impl From<&SkillPrepareResponse> for Packet {
    fn from(val: &SkillPrepareResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.player_id);
        packet.write_u8(val.unknown);
        if let Some(skill_index) = val.skill_index {
            packet.write_u8(skill_index);
        }
        if let Some(target_id) = val.target_id {
            packet.write_u32(target_id);
        }
        packet
    }
}