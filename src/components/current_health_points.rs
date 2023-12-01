use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct CurrentHealthPoints {
    pub current_health_points: u32
}

impl From<&PlayerRow> for CurrentHealthPoints {
    fn from(player_row: &PlayerRow) -> Self {
        CurrentHealthPoints {
            current_health_points: player_row.current_health_points,
        }
    }
}