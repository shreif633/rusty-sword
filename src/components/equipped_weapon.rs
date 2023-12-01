use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct EquippedWeapon {
    pub item: Option<Entity>
}

impl From<&PlayerRow> for EquippedWeapon {
    fn from(_player_row: &PlayerRow) -> Self {
        EquippedWeapon {
            item: None 
        }
    }
}