use bevy::prelude::*;

#[derive(Component)]
pub struct Appearence {
    pub name: String,
    pub face: u8,
    pub hair: u8,
    pub weapon_index: u16, 
    pub shield_index: u16, 
    pub helmet_index: u16, 
    pub chest_index: u16, 
    pub shorts_index: u16, 
    pub gloves_index: u16, 
    pub boots_index: u16, 
}