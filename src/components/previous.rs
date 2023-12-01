use bevy::prelude::*;

#[derive(Component)]
pub struct Previous<T> {
    pub entity: T
}

impl<T> From<T> for Previous<T> {
    fn from(entity: T) -> Self {
        Previous { entity }
    }
}