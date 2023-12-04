use bevy::prelude::*;
use crate::bundles::npc::NPCBundle;
use crate::components::npc::NPC;
use crate::components::direction::Direction;
use crate::configs::npcs::NPCsConfig;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::responses::npc_appear::NPCAppearResponse;
use super::tcp_server::SocketWriter;

pub struct SpawnNPCsPlugin;

impl Plugin for SpawnNPCsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npcs);
        app.add_systems(Update, handle_position_change);
    }
}

fn spawn_npcs(mut commands: Commands, npcs_configs: Res<NPCsConfig>) {
    for (_npc_index, npc_config) in npcs_configs.config.iter() {
        commands.spawn(NPCBundle::from(npc_config));
    }
}

fn handle_position_change(moved_query: Query<(&Previous<Position>, &Position, &SocketWriter), Changed<Position>>, npcs_query: Query<(Entity, &NPC, &Position, &Direction)>) {
    for (moved_previous_position, moved_position, moved_socket_writer) in &moved_query {
        for (entity, npc, position, direction) in &npcs_query {
            if !position.is_in_sight(&moved_previous_position.entity) {
                if position.is_in_sight(&moved_position) {
                    let npc_appear = NPCAppearResponse::new(entity, &npc, &position, &direction);
                    moved_socket_writer.write(&mut (&npc_appear).into());
                }
            }
        }
    }
}