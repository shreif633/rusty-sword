use bevy::prelude::*;
use crate::components::id::Id;
use crate::components::observers::Observers;
use crate::components::speed::Speed;
use crate::repositories::player::PlayerRow;
use crate::components::equipped_weapon::EquippedWeapon;
use crate::components::appearance::Appearance;
use crate::components::magical_attack::MagicalAttack;
use crate::components::physical_attack::PhysicalAttack;
use crate::components::rage::Rage;
use crate::components::experience::Experience;
use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::maximum_magic_points::MaximumMagicPoints;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::components::final_points::FinalPoints;
use crate::components::extra_points::ExtraPoints;
use crate::components::base_points::BasePoints;
use crate::components::player::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    id: Id,
    player: Player,
    base_points: BasePoints,
    extra_points: ExtraPoints,
    final_points: FinalPoints,
    previous_position: Previous<Position>,
    position: Position,
    maximum_health_points: MaximumHealthPoints,
    current_health_points: CurrentHealthPoints,
    previous_current_health_points: Previous<CurrentHealthPoints>,
    maximum_magic_points: MaximumMagicPoints,
    current_magic_points: CurrentMagicPoints,
    experience: Experience,
    rage: Rage,
    physical_attack: PhysicalAttack,
    magical_attack: MagicalAttack,
    appearence: Appearance,
    equipped_weapon: EquippedWeapon,
    previous_equipped_weapon: Previous<EquippedWeapon>,
    observers: Observers,
    speed: Speed,
}

impl PlayerBundle {
    pub fn new(player_row: &PlayerRow) -> Self {
        PlayerBundle {
            id: Id { id: player_row.id },
            player: Player::from(player_row),
            base_points: BasePoints::from(player_row),
            extra_points: ExtraPoints::from(player_row),
            final_points: FinalPoints::from(player_row),
            previous_position: Previous::from(Position { x: 0, y: 0, z: 0 }),
            position: Position::from(player_row),
            maximum_health_points: MaximumHealthPoints::from(player_row),
            current_health_points: CurrentHealthPoints::from(player_row),
            previous_current_health_points: Previous::from(CurrentHealthPoints::from(player_row)),
            maximum_magic_points: MaximumMagicPoints::from(player_row),
            current_magic_points: CurrentMagicPoints::from(player_row),
            experience: Experience::from(player_row),
            rage: Rage::from(player_row),
            physical_attack: PhysicalAttack::from(player_row),
            magical_attack: MagicalAttack::from(player_row),
            appearence: Appearance::from(player_row),
            equipped_weapon: EquippedWeapon::from(player_row),
            previous_equipped_weapon: Previous::from(EquippedWeapon::from(player_row)),
            observers: Observers::new(),
            speed: Speed { speed: 0 }
        }
    }
}