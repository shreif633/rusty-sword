use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Spawn {
    pub bottom_x: u32,
    pub bottom_y: u32,
    pub top_x: u32,
    pub top_y: u32,
}