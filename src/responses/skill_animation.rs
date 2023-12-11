use crate::enums::damage_type::DamageType;
use crate::enums::target_type::TargetType;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 63;

#[derive(Debug)]
pub struct SkillAnimationResponse {
    pub skill_index: u8,
    pub player_id: i32,
    pub target_id: i32,
    pub target_type: TargetType,
    pub animation_index: u8,
    pub normal_damage: Option<u16>,
    pub explosive_blow_damage: Option<u16>,
    pub damage_type: Option<DamageType>,
    pub soul_pocket_damage: Option<u16>
}

impl From<&mut Packet> for SkillAnimationResponse {
    fn from(packet: &mut Packet) -> Self {
        let skill_index = packet.get_u8();
        let player_id = packet.get_i32();
        let target_id = packet.get_i32();
        let target_type = packet.get_u8();
        let target_type = match target_type {
            0 => TargetType::Player,
            _ => TargetType::Monster
        };
        let unknown = packet.get_u8();
        let (normal_damage, explosive_blow_damage, damage_type, soul_pocket_damage) = if packet.size() == 14 {
            (None, None, None, None)
        } else {
            let normal_damage = packet.get_u16();
            let explosive_blow_damage = packet.get_u16();
            let damage_type = packet.get_u8();
            let damage_type = match damage_type {
                0 => DamageType::Miss,
                3 => DamageType::Critical,
                _ => DamageType::Normal
            };
            let soul_pocket_damage = packet.get_u16();
            (Some(normal_damage), Some(explosive_blow_damage), Some(damage_type), Some(soul_pocket_damage))
        };
        SkillAnimationResponse { 
            skill_index, player_id, target_id, target_type, animation_index: unknown, 
            normal_damage, explosive_blow_damage, damage_type, soul_pocket_damage 
        }
    }
}

impl From<&SkillAnimationResponse> for Packet {
    fn from(val: &SkillAnimationResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(val.skill_index);
        packet.write_i32(val.player_id);
        packet.write_i32(val.target_id);
        match val.target_type {
            TargetType::Player => packet.write_u8(0),
            TargetType::Monster => packet.write_u8(1) 
        };
        packet.write_u8(val.animation_index);
        if let Some(normal_damage) = val.normal_damage {
            packet.write_u16(normal_damage);
        }
        if let Some(explosive_blow_damage) = val.explosive_blow_damage {
            packet.write_u16(explosive_blow_damage);
        }
        if let Some(damage_type) = &val.damage_type {
            match damage_type {
                DamageType::Miss => packet.write_u8(0),
                DamageType::Critical => packet.write_u8(3),
                DamageType::Normal => packet.write_u8(1),
            };
        }
        if let Some(soul_pocket_damage) = val.soul_pocket_damage {
            packet.write_u16(soul_pocket_damage);
        }
        packet
    }
}