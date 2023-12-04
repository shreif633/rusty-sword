use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct Level {
    pub level: u8,
}

impl From<&PlayerRow> for Level {
    fn from(player_row: &PlayerRow) -> Self {
        Level {
            level: player_row.level
        }
    }
}