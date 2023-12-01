use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct BasePoints {
    pub base_strength: u16, 
    pub base_health: u16, 
    pub base_intelligence: u16, 
    pub base_wisdom: u16,
    pub base_agility: u16,  
}

impl From<&PlayerRow> for BasePoints {
    fn from(player_row: &PlayerRow) -> Self {
        BasePoints {
            base_strength: player_row.base_strength,
            base_health: player_row.base_health,
            base_intelligence: player_row.base_intelligence,
            base_wisdom: player_row.base_wisdom,
            base_agility: player_row.base_agility,
        }
    }
}