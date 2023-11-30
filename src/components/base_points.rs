use bevy::prelude::*;

#[derive(Component)]
pub struct BasePoints {
    pub base_strength: u16, 
    pub base_health: u16, 
    pub base_intelligence: u16, 
    pub base_wisdom: u16,
    pub base_agility: u16,  
}