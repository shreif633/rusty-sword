use bevy::prelude::*;
use crate::repositories::player::PlayerRow;
use crate::enums::player_class::PlayerClass;

#[derive(Component)]
pub struct Job {
    pub level: u8,
    pub class: PlayerClass,
    pub specialty: u8,
}

impl From<&PlayerRow> for Job {
    fn from(player_row: &PlayerRow) -> Self {
        Job {
            class: PlayerClass::from(player_row.class),
            level: player_row.level,
            specialty: player_row.specialty
        }
    }
}