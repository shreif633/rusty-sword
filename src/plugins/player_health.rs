use bevy::prelude::*;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::previous::Previous;
use crate::responses::player_current_health_points::PlayerCurrentHealthPointsResponse;
use super::tcp_server::SocketWriter;

pub struct PlayerHealthPlugin;

impl Plugin for PlayerHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, limit_current_health_points);
        app.add_systems(Last, update_player_health);
    }
}

fn limit_current_health_points(mut query: Query<(&mut CurrentHealthPoints, &MaximumHealthPoints)>) {
    for (mut current_health_points, maximum_health_points) in query.iter_mut() {
        if current_health_points.current_health_points > maximum_health_points.maximum_health_points {
           current_health_points.current_health_points = maximum_health_points.maximum_health_points;
        }
    }
}

fn update_player_health(mut query: Query<(Changed<CurrentHealthPoints>, &mut Previous<CurrentHealthPoints>, &CurrentHealthPoints, &SocketWriter)>) {
    for (changed, mut previous_health_points, current_health_points, socket_writer) in query.iter_mut() {
        if changed {
            previous_health_points.entity.current_health_points = current_health_points.current_health_points;
            let player_current_health_points_response = PlayerCurrentHealthPointsResponse { current_health_points: current_health_points.current_health_points };
            socket_writer.write(&mut (&player_current_health_points_response).into());
        }
    }
}