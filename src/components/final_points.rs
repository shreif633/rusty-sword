use bevy::prelude::*;

#[derive(Component)]
pub struct FinalPoints {
    pub on_target_point: u16, 
    pub evasion: u16, 
    pub defense: u16, 
    pub absorption: u16, 
    pub fire_resistence: u16, 
    pub ice_resistence: u16, 
    pub lighning_resistence: u16,
    pub curse_resistence: u16, 
    pub non_elemental_resistence: u16,
}