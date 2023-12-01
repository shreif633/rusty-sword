use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct MagicalAttack {
    pub minimum_magical_attack: u16,
    pub maximum_magical_attack: u16,
}

impl From<&PlayerRow> for MagicalAttack {
    fn from(player_row: &PlayerRow) -> Self {
        MagicalAttack {
            minimum_magical_attack: player_row.minimum_magical_attack,
            maximum_magical_attack: player_row.minimum_magical_attack,
        }
    }
}