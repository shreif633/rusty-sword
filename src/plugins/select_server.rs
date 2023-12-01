use bevy::prelude::*;
use crate::responses::server_selected::ServerSelectedResponse;
use crate::responses::check_hash::CheckHashResponse;
use crate::responses::analyze::AnalyzeResponse;
use crate::requests::server_select::ServerSelectRequest;
use super::tcp_server::SocketWriter;

pub struct ServerSelectPlugin;

impl Plugin for ServerSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_server_selection);
    }
}

fn handle_server_selection(mut commands: Commands, query: Query<(Entity, &SocketWriter), With<ServerSelectRequest>>) {
    let analyze = AnalyzeResponse::new();
    let check_hash = CheckHashResponse::new();
    let server_selected = ServerSelectedResponse::new();
    for (entity, socket_writer) in &query {
        socket_writer.write(&mut (&analyze).into());
        socket_writer.write(&mut (&check_hash).into());
        socket_writer.write(&mut (&server_selected).into());
        commands.entity(entity).remove::<ServerSelectRequest>();
    }
}