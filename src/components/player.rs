use bevy::prelude::*;
use crate::repositories::player::PlayerRow;
use crate::enums::player_class::PlayerClass;

#[derive(Component)]
pub struct Player {
    pub class: PlayerClass,
    pub specialty: u8,
}

impl From<&PlayerRow> for Player {
    fn from(player_row: &PlayerRow) -> Self {
        Player {
            class: PlayerClass::from(player_row.class),
            specialty: player_row.specialty
        }
    }
}