use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct CurrentMagicPoints {
    pub current_magic_points: u16
}

impl From<&PlayerRow> for CurrentMagicPoints {
    fn from(player_row: &PlayerRow) -> Self {
        CurrentMagicPoints {
            current_magic_points: player_row.current_magic_points,
        }
    }
}