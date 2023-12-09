use bevy::prelude::*;
use crate::components::aggro::Aggro;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::experience::Experience;
use crate::components::id::Id;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::observers::Observers;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::components::monster::Monster;
use crate::components::spawn::Spawn;

#[derive(Bundle)]
pub struct MonsterBundle {
    pub id: Id,
    pub monster: Monster,
    pub experience: Experience,
    pub previous_position: Previous<Position>,
    pub position: Position,
    pub maximum_health_points: MaximumHealthPoints,
    pub current_health_points: CurrentHealthPoints,
    pub previous_current_health_points: Previous<CurrentHealthPoints>,
    pub spawn: Spawn,
    pub observers: Observers,
    pub aggro: Aggro,
}