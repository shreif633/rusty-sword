use bevy::prelude::*;
use crate::components::npc::Npc;
use crate::components::position::Position;
use crate::components::direction::Direction;
use crate::configs::npcs::NpcConfig;

#[derive(Bundle)]
pub struct NpcBundle {
    pub npc: Npc,
    pub position: Position,
    pub direction: Direction,
}


impl NpcBundle {
    pub fn from(npc_config: &NpcConfig) -> Self {
        NpcBundle {
            npc: Npc::from(npc_config),
            position: Position::from(npc_config),
            direction: Direction::from(npc_config),
        }
    }
}