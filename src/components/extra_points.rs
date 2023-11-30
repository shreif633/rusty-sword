use bevy::prelude::*;

#[derive(Component)]
pub struct ExtraPoints {
    pub extra_strength: u16, 
    pub extra_health: u16, 
    pub extra_intelligence: u16, 
    pub extra_wisdom: u16,
    pub extra_agility: u16,  
}