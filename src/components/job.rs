use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct Job {
    pub level: u8,
    pub class: u8,
    pub specialty: u8,
}

impl From<&PlayerRow> for Job {
    fn from(player_row: &PlayerRow) -> Self {
        Job {
            class: player_row.class,
            level: player_row.level,
            specialty: player_row.specialty
        }
    }
}