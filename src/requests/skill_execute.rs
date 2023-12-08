use bevy::prelude::*;
use crate::framework::packet::Packet;
use crate::enums::target_type::TargetType;

pub const HEADER: u8 = 117;

#[derive(Component, Debug, Clone)]
pub struct SkillExecuteRequest {
    pub skill_index: u8,
    pub target_type: Option<TargetType>,
    pub target_id: Option<i32>,
}

impl From<&mut Packet> for SkillExecuteRequest {
    fn from(packet: &mut Packet) -> Self {
        let skill_id = packet.get_u8();
        let (target_type, target_id) = if packet.size() == 9 {
            let target_type = TargetType::from(packet.get_u8());
            let target_id = packet.get_i32();
            (Some(target_type), Some(target_id))
        } else {
            (None, None)
        };
        SkillExecuteRequest { skill_index: skill_id, target_type, target_id }
    }
}