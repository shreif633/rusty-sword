use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component)]
pub struct Aggro {
    pub list: HashMap<Entity, u32>
}

impl Aggro {
    pub fn new() -> Self {
        Aggro { list: HashMap::new() }
    }

    pub fn add(&mut self, entity: Entity, value: u32) {
        if let Some(old_value) = self.list.get(&entity) {
            self.list.insert(entity, old_value + value);
        } else {
            self.list.insert(entity, value);
        }
    }
}