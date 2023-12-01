use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct Player {
    pub id: u32
}

impl From<&PlayerRow> for Player {
    fn from(player_row: &PlayerRow) -> Self {
        Player { id: player_row.id }
    }
}