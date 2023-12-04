use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct Id {
    pub id: i32, 
}

impl From<&PlayerRow> for Id {
    fn from(player_row: &PlayerRow) -> Self {
        Id {
            id: player_row.id,
        }
    }
}