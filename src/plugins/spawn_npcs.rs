use bevy::prelude::*;
use crate::bundles::npc::NpcBundle;
use crate::configs::npcs::NpcsConfig;

pub struct SpawnNpcsPlugin;

impl Plugin for SpawnNpcsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npcs);
    }
}

fn spawn_npcs(mut commands: Commands, npcs_configs: Res<NpcsConfig>) {
    for (npc_index, npc_config) in npcs_configs.config.iter() {
        let npc_id: i32 = (*npc_index).try_into().unwrap();
        commands.spawn(NpcBundle::new(npc_id, npc_config));
    }
}