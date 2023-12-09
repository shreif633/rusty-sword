use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct FinalPoints {
    pub on_target_point: u16, 
    pub evasion: u16, 
    pub defense: u16, 
    pub absorption: u8, 
    pub fire_resistence: u16, 
    pub ice_resistence: u16, 
    pub lighning_resistence: u16,
    pub curse_resistence: u16, 
    pub non_elemental_resistence: u16,
}

impl From<&PlayerRow> for FinalPoints {
    fn from(player_row: &PlayerRow) -> Self {
        FinalPoints {
            on_target_point: player_row.on_target_point,
            evasion: player_row.evasion,
            defense: player_row.defense,
            absorption: player_row.absorption,
            fire_resistence: player_row.fire_resistence,
            ice_resistence: player_row.ice_resistence,
            lighning_resistence: player_row.lighning_resistence,
            curse_resistence: player_row.curse_resistence,
            non_elemental_resistence: player_row.non_elemental_resistence,
        }
    }
}