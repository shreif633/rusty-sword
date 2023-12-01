use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct MaximumHealthPoints {
    pub maximum_health_points: u32
}

impl From<&PlayerRow> for MaximumHealthPoints {
    fn from(player_row: &PlayerRow) -> Self {
        MaximumHealthPoints {
            maximum_health_points: player_row.maximum_health_points,
        }
    }
}