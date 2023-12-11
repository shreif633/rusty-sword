use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
    pub animation_index: u8,
    pub skill_index: Option<u8>,
    pub target: Option<Entity>,
}

impl Animation {
    pub fn with_target(animation_index: u8, skill_index: u8, target: Entity) -> Self {
        Animation {
            animation_index,
            skill_index: Some(skill_index),
            target: Some(target),
        }
    }

    pub fn without_target(animation_index: u8) -> Self {
        Animation {
            animation_index,
            skill_index: None,
            target: None,
        }
    }
}