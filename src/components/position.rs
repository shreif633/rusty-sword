use bevy::prelude::*;
use crate::configs::npcs::NpcConfig;
use crate::repositories::player::PlayerRow;
use rand::{thread_rng, Rng};
use super::spawn::Spawn;

#[derive(Component, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl From<&PlayerRow> for Position {
    fn from(player_row: &PlayerRow) -> Self {
        Position { 
            x: player_row.x, 
            y: player_row.y, 
            z: player_row.z
        }
    }
}

impl From<&NpcConfig> for Position {
    fn from(npc_config: &NpcConfig) -> Self {
        Position { 
            x: npc_config.x, 
            y: npc_config.y, 
            z: npc_config.z
        }
    }
}

impl From<&Spawn> for Position {
    fn from(spawn: &Spawn) -> Self {
        let mut rng = thread_rng();
        let random_x = rng.gen_range(spawn.bottom_x..spawn.top_x);
        let random_y = rng.gen_range(spawn.bottom_y..spawn.top_y);
        Position { 
            x: random_x, 
            y: random_y, 
            z: 0
        }
    }
}

impl Position {
    pub fn calculate_distance(&self, other: &Position) -> u32 {
        let x_diff = self.x as f64 - other.x as f64;
        let y_diff = self.y as f64 - other.y as f64;
        // Euclidean distance formula: sqrt((x2 - x1)^2 + (y2 - y1)^2)
        (x_diff.powi(2) + y_diff.powi(2)).sqrt().round() as u32
    }

    pub fn is_in_range(&self, other: &Position, range: u32) -> bool {
        self.calculate_distance(other) < range
    }

    pub fn is_in_sight(&self, other: &Position) -> bool {
        self.is_in_range(other, 900)
    }
}