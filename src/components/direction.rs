use bevy::prelude::*;
use crate::configs::npcs::NpcConfig;

#[derive(Component)]
pub struct Direction {
    pub direction: u16,
}

impl Direction {
    pub fn new(x: u32, y: u32, look_at_x: u32, look_at_y: u32) -> Self {
        let n_x = look_at_x as i32 - x as i32;
        let n_y = look_at_y as i32 - y as i32;
        Direction {
            direction: calculate_direction(n_x, n_y)
        }
    }
}

impl From<&NpcConfig> for Direction {
    fn from(npc_config: &NpcConfig) -> Self {
        Direction::new(npc_config.x, npc_config.y, npc_config.look_at_x, npc_config.look_at_y)
    }
}

fn calculate_direction(n_x: i32, n_y: i32) -> u16 {
    if n_x == 0 && n_y == 0 {
        return 0;
    }
    let (mut new_x, mut new_y) = (n_x, n_y);
    if new_x.abs() >= new_y.abs() && new_x.abs() > 127 {
        new_y = 127 * n_y / new_x.abs();
        new_x = if n_x <= 0 { -1 } else { 1 } * 254 - 127;
    } else if new_x.abs() < new_y.abs() && new_y.abs() > 127 {
        new_x = 127 * n_x / new_y.abs();
        new_y = if n_y <= 0 { -1 } else { 1 } * 254 - 127;
    }
    (new_y + (new_x * 256)) as u16
}