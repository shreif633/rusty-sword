use bevy::prelude::*;

#[derive(Component)]
pub struct Medicine {
    pub health_recovered: u32,
    pub cooldown_in_seconds: f32
}