use bevy::prelude::*;
use crate::configs::npcs::NPCConfig;

#[derive(Component)]
pub struct NPC {
    pub index: u16,
    pub shape: u8,
}

impl From<&NPCConfig> for NPC {
    fn from(npc_config: &NPCConfig) -> Self {
        NPC { 
            index: npc_config.index, 
            shape: npc_config.shape
        }
    }
}