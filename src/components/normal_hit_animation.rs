use bevy::prelude::*;
use crate::enums::damage_type::DamageType;

#[derive(Component)]
pub struct NormalHitAnimation { 
    pub target: Entity,
    pub normal_damage: u32,
    pub explosive_blow_damage: u32,
    pub damage_type: DamageType,
    pub soul_pocket_damage: u32,
}

impl NormalHitAnimation {
    pub fn new(target: Entity, normal_damage: u32, explosive_blow_damage: u32, damage_type: DamageType, soul_pocket_damage: u32) -> Self {
        NormalHitAnimation {
            target,
            normal_damage,
            explosive_blow_damage,
            damage_type,
            soul_pocket_damage,
        }
    }
}