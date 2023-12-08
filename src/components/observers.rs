use bevy::prelude::*;

#[derive(Component)]
pub struct Observers {
    pub entities: Vec<Entity>,
}

impl Observers {
    pub fn new() -> Self {
        Observers { entities: vec![] }
    }
}