use bevy::prelude::*;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::components::monster::Monster;

#[derive(Bundle)]
pub struct MonsterBundle {
    pub monster: Monster,
    pub previous_position: Previous<Position>,
    pub position: Position,
    pub maximum_health_points: MaximumHealthPoints,
    pub current_health_points: CurrentHealthPoints,
    pub previous_current_health_points: Previous<CurrentHealthPoints>,
}