use bevy::prelude::*;
use crate::components::id::Id;
use crate::components::npc::Npc;
use crate::components::observers::Observers;
use crate::components::position::Position;
use crate::components::direction::Direction;
use crate::configs::npcs::NpcConfig;

#[derive(Bundle)]
pub struct NpcBundle {
    pub id: Id,
    pub npc: Npc,
    pub position: Position,
    pub direction: Direction,
    pub observers: Observers,
}


impl NpcBundle {
    pub fn new(id: i32, npc_config: &NpcConfig) -> Self {
        NpcBundle {
            id: Id { id },
            npc: Npc::from(npc_config),
            position: Position::from(npc_config),
            direction: Direction::from(npc_config),
            observers: Observers::new()
        }
    }
}