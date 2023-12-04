use crate::framework::packet::Packet;

pub const HEADER: u8 = 81;

#[derive(Debug, Clone)]
pub struct SkillPrepareRequest {
    pub skill_index: u8,
    pub target_id: i32,
}

impl From<&mut Packet> for SkillPrepareRequest {
    fn from(packet: &mut Packet) -> Self {
        let skill_index = packet.get_u8();
        let target_id = packet.get_i32();
        SkillPrepareRequest { skill_index, target_id }
    }
}