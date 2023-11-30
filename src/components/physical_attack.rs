use bevy::prelude::*;

#[derive(Component)]
pub struct PhysicalAttack {
    pub minimum_physical_attack: u16,
    pub maximum_physical_attack: u16,
}