use bevy::prelude::*;

#[derive(Component)]
pub struct EquippedWeapon {
    pub item: Option<Entity>
}