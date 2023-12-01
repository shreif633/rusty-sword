use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct Rage {
    pub rage: u32,
}

impl From<&PlayerRow> for Rage {
    fn from(player_row: &PlayerRow) -> Self {
        Rage { 
            rage: player_row.rage
        }
    }
}