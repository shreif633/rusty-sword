use bevy::prelude::*;

#[derive(Component)]
pub struct NetworkObservers {
    pub entities: Vec<Entity>,
}

impl NetworkObservers {
    pub fn new() -> Self {
        NetworkObservers { entities: vec![] }
    }
}