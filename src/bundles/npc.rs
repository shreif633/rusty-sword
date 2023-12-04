use bevy::prelude::*;
use crate::components::npc::NPC;
use crate::components::position::Position;
use crate::components::direction::Direction;
use crate::configs::npcs::NPCConfig;

#[derive(Bundle)]
pub struct NPCBundle {
    pub npc: NPC,
    pub position: Position,
    pub direction: Direction,
}


impl NPCBundle {
    pub fn from(npc_config: &NPCConfig) -> Self {
        NPCBundle {
            npc: NPC::from(npc_config),
            position: Position::from(npc_config),
            direction: Direction::from(npc_config),
        }
    }
}