use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct Experience {
    pub experience: i64,
}

impl From<&PlayerRow> for Experience {
    fn from(player_row: &PlayerRow) -> Self {
        Experience { 
            experience: player_row.experience
        }
    }
}