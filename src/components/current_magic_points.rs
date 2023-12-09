use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct CurrentMagicPoints {
    pub current_magic_points: u16
}

impl CurrentMagicPoints {
    pub fn sub(&mut self, value: u16) {
        if value > self.current_magic_points {
            self.current_magic_points = 0;
        } else {
            self.current_magic_points -= value;
        }
    }
}


impl From<&PlayerRow> for CurrentMagicPoints {
    fn from(player_row: &PlayerRow) -> Self {
        CurrentMagicPoints {
            current_magic_points: player_row.current_magic_points,
        }
    }
}