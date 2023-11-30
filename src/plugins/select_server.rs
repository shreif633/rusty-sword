use bevy::prelude::*;
use crate::packets::server::server_selected::ServerSelected;
use crate::packets::server::check_hash::CheckHash;
use crate::packets::server::analyze::Analyze;
use crate::packets::client::server_select::ServerSelect;
use super::tcp_server::SocketWriter;

pub struct ServerSelectPlugin;

impl Plugin for ServerSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_server_selection);
    }
}

fn handle_server_selection(mut commands: Commands, query: Query<(Entity, &SocketWriter), With<ServerSelect>>) {
    let analyze = Analyze::new();
    let check_hash = CheckHash::new();
    let server_selected = ServerSelected::new();
    for (entity, socket_writer) in &query {
        socket_writer.write(&mut (&analyze).into());
        socket_writer.write(&mut (&check_hash).into());
        socket_writer.write(&mut (&server_selected).into());
        commands.entity(entity).remove::<ServerSelect>();
    }
}