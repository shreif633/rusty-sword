use bevy::prelude::*;

#[derive(Component)]
pub struct Damage {
    pub source: Entity,
    pub target: Entity,
    pub damage: u32,
    pub aggro_multiplier: f32,
    pub skill_index: Option<u8>,
    pub animation: Option<u8>,
}