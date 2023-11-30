use bevy::prelude::*;

#[derive(Component)]
pub struct Job {
    pub level: u8,
    pub class: u8,
    pub specialty: u8,
}