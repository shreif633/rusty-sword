use bevy::prelude::*;
use crate::configs::npcs::NpcConfig;

#[derive(Component)]
pub struct Npc {
    pub index: u16,
    pub shape: u8,
}

impl From<&NpcConfig> for Npc {
    fn from(npc_config: &NpcConfig) -> Self {
        Npc { 
            index: npc_config.index, 
            shape: npc_config.shape
        }
    }
}