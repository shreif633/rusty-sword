use bevy::prelude::*;
use std::time::Duration;
use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::id::Id;
use crate::components::observers::Observers;
use crate::components::speed::Speed;
use crate::requests::skill_execute::SkillExecuteRequest;
use crate::responses::general_state::GeneralStateResponse;
use super::tcp_server::SocketWriter;

const MANA_PER_TICK: u16 = 50;

pub struct SpeedPlugin;

impl Plugin for SpeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_skill_execute);
        app.add_systems(Update, consume_running_mana);
        app.add_systems(Last, broadcast_speed_changed);
    }
}

#[derive(Component)]
struct Running {
    consume_mana_timer: Timer
}

fn handle_skill_execute(mut commands: Commands, mut query: Query<(Entity, &SkillExecuteRequest, Option<&Running>, &mut Speed, &CurrentMagicPoints)>) {
    for (entity, client_packet, optional_running, mut speed, current_magic_points) in query.iter_mut() {
        if client_packet.skill_index == 0 {
            if optional_running.is_some() {
                commands.entity(entity).remove::<Running>();
                speed.speed = 0;
            } else {
                if current_magic_points.current_magic_points >= MANA_PER_TICK {
                    commands.entity(entity).insert(Running { consume_mana_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating) });
                    speed.speed = 45;
                }
            }
        }
    }
}

fn consume_running_mana(mut commands: Commands, mut query: Query<(Entity, &mut CurrentMagicPoints, &mut Running, &mut Speed)>, time: Res<Time>) {
    for (entity, mut current_magic_points, mut running, mut speed) in query.iter_mut() {
        running.consume_mana_timer.tick(time.delta());
        if running.consume_mana_timer.just_finished() {
            current_magic_points.sub(MANA_PER_TICK);
            if current_magic_points.current_magic_points < MANA_PER_TICK {
                commands.entity(entity).remove::<Running>();
                speed.speed = 0;
            }
        }
    }
}

fn broadcast_speed_changed(mut players: Query<(&Id, &Speed, &Observers), Changed<Speed>>, observers: Query<&SocketWriter>) {
    for (id, speed, player_observers) in players.iter_mut() {
        let response = GeneralStateResponse { target_id: id.id, general_state: 0, speed: Some(speed.speed) };
        for entity in &player_observers.entities {
            if let Ok(observer_socket_writer) = observers.get(*entity) {
                observer_socket_writer.write(&mut (&response).into());
            }
        }
    }
}