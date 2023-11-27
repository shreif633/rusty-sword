use bevy::prelude::*;
use crate::packets::{client::server_select::ServerSelect, server::{analyze::Analyze, check_hash::CheckHash, server_selected::ServerSelected}};
use super::tcp_server::SocketWriter;

pub struct ServerSelectPlugin;

impl Plugin for ServerSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_server_selection);
    }
}

fn handle_server_selection(mut commands: Commands, query: Query<(Entity, &SocketWriter), With<ServerSelect>>) {
    let analyze = Analyze { unknown: vec![211, 0, 0, 0, 28, 207, 86, 101, 0, 128, 3, 0] };
    let check_hash = CheckHash { hash: 1325039837 };
    let server_selected = ServerSelected { unknown: vec![242, 108, 141, 16, 54, 212, 76, 126, 68, 30, 207, 86, 101, 97, 30, 0, 0, 118, 0, 0, 0, 0, 0, 0, 0, 2, 18, 2] };
    for (entity, socket_writer) in &query {
        socket_writer.write(&mut (&analyze).into());
        socket_writer.write(&mut (&check_hash).into());
        socket_writer.write(&mut (&server_selected).into());
        commands.entity(entity).remove::<ServerSelect>();
    }
}