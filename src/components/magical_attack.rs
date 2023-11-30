use bevy::prelude::*;

#[derive(Component)]
pub struct MagicalAttack {
    pub minimum_magical_attack: u16,
    pub maximum_magical_attack: u16,
}