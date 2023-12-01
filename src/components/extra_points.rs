use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct ExtraPoints {
    pub extra_strength: u16, 
    pub extra_health: u16, 
    pub extra_intelligence: u16, 
    pub extra_wisdom: u16,
    pub extra_agility: u16,  
}

impl From<&PlayerRow> for ExtraPoints {
    fn from(player_row: &PlayerRow) -> Self {
        ExtraPoints {
            extra_strength: player_row.extra_strength,
            extra_health: player_row.extra_health,
            extra_intelligence: player_row.extra_intelligence,
            extra_wisdom: player_row.extra_wisdom,
            extra_agility: player_row.extra_agility,
        }
    }
}