use bevy::prelude::*;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::maximum_magic_points::MaximumMagicPoints;
use crate::components::network_writer::NetworkWriter;
use crate::components::previous::Previous;
use crate::responses::player_current_health_points::PlayerCurrentHealthPointsResponse;
use crate::responses::player_current_magic_points::PlayerCurrentMagicPointsResponse;

pub struct PlayerHealthPlugin;

impl Plugin for PlayerHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, limit_current_health_points);
        app.add_systems(Last, broadcast_player_health);
        app.add_systems(PostUpdate, limit_current_magic_points);
        app.add_systems(Last, broadcast_player_magic);
    }
}

fn limit_current_health_points(mut query: Query<(&mut CurrentHealthPoints, &MaximumHealthPoints)>) {
    for (mut current_health_points, maximum_health_points) in query.iter_mut() {
        if current_health_points.current_health_points > maximum_health_points.maximum_health_points {
           current_health_points.current_health_points = maximum_health_points.maximum_health_points;
        }
    }
}

fn broadcast_player_health(mut query: Query<(&mut Previous<CurrentHealthPoints>, &CurrentHealthPoints, &NetworkWriter), Changed<CurrentHealthPoints>>) {
    for (mut previous_health_points, current_health_points, socket_writer) in query.iter_mut() {
        previous_health_points.entity.current_health_points = current_health_points.current_health_points;
        let player_current_health_points_response = PlayerCurrentHealthPointsResponse { current_health_points: current_health_points.current_health_points };
        socket_writer.write(&mut (&player_current_health_points_response).into());
    }
}

fn limit_current_magic_points(mut query: Query<(&mut CurrentMagicPoints, &MaximumMagicPoints)>) {
    for (mut current_magic_points, maximum_magic_points) in query.iter_mut() {
        if current_magic_points.current_magic_points > maximum_magic_points.maximum_magic_points {
           current_magic_points.current_magic_points = maximum_magic_points.maximum_magic_points;
        }
    }
}

fn broadcast_player_magic(query: Query<(&CurrentMagicPoints, &NetworkWriter), Changed<CurrentMagicPoints>>) {
    for (current_magic_points, socket_writer) in query.iter() {
        let player_current_magic_points_response = PlayerCurrentMagicPointsResponse { current_magic_points: current_magic_points.current_magic_points };
        socket_writer.write(&mut (&player_current_magic_points_response).into());
    }
}