use bevy::prelude::*;
use crate::enums::damage_type::DamageType;
use crate::enums::target_type::TargetType;

#[derive(Component)]
pub struct SkillAnimation { 
    pub skill_index: u8,
    pub target: Entity,
    pub target_type: TargetType,
    pub animation_index: u8,
    pub normal_damage: Option<u16>,
    pub explosive_blow_damage: Option<u16>,
    pub damage_type: Option<DamageType>,
    pub soul_pocket_damage: Option<u16>
}

impl SkillAnimation {
    pub fn without_damage(skill_index: u8, target: Entity, target_type: TargetType, animation_index: u8) -> Self {
        SkillAnimation {
            skill_index,
            target,
            target_type,
            animation_index,
            normal_damage: None,
            explosive_blow_damage: None,
            damage_type: None,
            soul_pocket_damage: None,
        }
    }

    pub fn with_damage(skill_index: u8, target: Entity, target_type: TargetType, animation_index: u8, normal_damage: u16, explosive_blow_damage: u16, damage_type: DamageType, soul_pocket_damage: u16) -> Self {
        SkillAnimation {
            skill_index,
            target,
            target_type,
            animation_index,
            normal_damage: Some(normal_damage),
            explosive_blow_damage: Some(explosive_blow_damage),
            damage_type: Some(damage_type),
            soul_pocket_damage: Some(soul_pocket_damage),
        }
    }
}