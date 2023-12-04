use bevy::prelude::*;
use crate::bundles::npc::NpcBundle;
use crate::components::id::Id;
use crate::components::npc::Npc;
use crate::components::direction::Direction;
use crate::configs::npcs::NpcsConfig;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::responses::npc_appear::NpcAppearResponse;
use super::tcp_server::SocketWriter;

pub struct SpawnNpcsPlugin;

impl Plugin for SpawnNpcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npcs);
        app.add_systems(Update, handle_position_change);
    }
}

fn spawn_npcs(mut commands: Commands, npcs_configs: Res<NpcsConfig>) {
    for (npc_index, npc_config) in npcs_configs.config.iter() {
        let npc_id: i32 = (*npc_index).try_into().unwrap();
        commands.spawn(NpcBundle::new(npc_id, npc_config));
    }
}

fn handle_position_change(moved_query: Query<(&Previous<Position>, &Position, &SocketWriter), Changed<Position>>, npcs_query: Query<(&Id, &Npc, &Position, &Direction)>) {
    for (moved_previous_position, moved_position, moved_socket_writer) in &moved_query {
        for (id, npc, position, direction) in &npcs_query {
            if !position.is_in_sight(&moved_previous_position.entity) && position.is_in_sight(moved_position) {
                let npc_appear = NpcAppearResponse::new(id, npc, position, direction);
                moved_socket_writer.write(&mut (&npc_appear).into());
            }
        }
    }
}