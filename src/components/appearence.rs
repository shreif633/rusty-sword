use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

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

impl From<&PlayerRow> for Appearence {
    fn from(player_row: &PlayerRow) -> Self {
        Appearence {
            name: player_row.name.clone(),
            face: player_row.face,
            hair: player_row.hair,
            weapon_index: player_row.weapon_index,
            shield_index: player_row.shield_index,
            helmet_index: player_row.helmet_index,
            chest_index: player_row.chest_index,
            shorts_index: player_row.shorts_index,
            gloves_index: player_row.gloves_index,
            boots_index: player_row.boots_index,
        }
    }
}