use bevy::prelude::*;
use std::{collections::hash_map::HashMap, marker::PhantomData};

#[derive(Resource)]
pub struct EntityMap<T> {
    pub map: HashMap<i32, Entity>,
    phanton_data: PhantomData<T>
}

impl<T> EntityMap<T> {
    pub fn new() -> Self {
        EntityMap { map: HashMap::new(), phanton_data: PhantomData }
    }
}