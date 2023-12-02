use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct PhysicalAttack {
    pub minimum_physical_attack: u16,
    pub maximum_physical_attack: u16,
}

impl From<&PlayerRow> for PhysicalAttack {
    fn from(player_row: &PlayerRow) -> Self {
        PhysicalAttack {
            minimum_physical_attack: player_row.minimum_physical_attack,
            maximum_physical_attack: player_row.maximum_physical_attack,
        }
    }
}