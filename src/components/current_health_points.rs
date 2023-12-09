use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct CurrentHealthPoints {
    pub current_health_points: u32
}

impl CurrentHealthPoints {
    pub fn sub(&mut self, value: u32) {
        if value > self.current_health_points {
            self.current_health_points = 0;
        } else {
            self.current_health_points -= value;
        }
    }
}

impl From<&PlayerRow> for CurrentHealthPoints {
    fn from(player_row: &PlayerRow) -> Self {
        CurrentHealthPoints {
            current_health_points: player_row.current_health_points,
        }
    }
}