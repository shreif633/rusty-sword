use bevy::prelude::*;
use crate::repositories::player::PlayerRow;

#[derive(Component)]
pub struct EquippedWeapon {
    pub item_id: Option<i32>
}

impl From<&PlayerRow> for EquippedWeapon {
    fn from(_player_row: &PlayerRow) -> Self {
        EquippedWeapon {
            item_id: None 
        }
    }
}