use crate::framework::packet::Packet;

pub const HEADER: u8 = 81;

#[derive(Debug, Clone)]
pub struct SkillPrepare {
    pub skill_index: u8,
    pub target_id: u32,
}

impl From<&mut Packet> for SkillPrepare {
    fn from(packet: &mut Packet) -> Self {
        let skill_index = packet.get_u8();
        let target_id = packet.get_u32();
        SkillPrepare { skill_index, target_id }
    }
}