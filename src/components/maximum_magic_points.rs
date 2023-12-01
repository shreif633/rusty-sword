use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct MaximumMagicPoints {
    pub maximum_magic_points: u16
}

impl From<&PlayerRow> for MaximumMagicPoints {
    fn from(player_row: &PlayerRow) -> Self {
        MaximumMagicPoints {
            maximum_magic_points: player_row.maximum_magic_points,
        }
    }
}