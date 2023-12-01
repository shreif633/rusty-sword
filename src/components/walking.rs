use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Walking {
    pub done: bool,
    pub delta_x: u8,
    pub delta_y: u8,
    pub delta_z: u8,
}