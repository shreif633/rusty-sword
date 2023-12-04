use bevy::prelude::*;
use crate::{components::{player::Player, visual_effect::VisualEffect, position::Position}, enums::target_type::TargetType, responses::visual_effect::VisualEffectResponse};
use super::tcp_server::SocketWriter;

pub struct VisualEffectPlugin;

impl Plugin for VisualEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, show_visual_effects);
    }
}

fn show_visual_effects(mut commands: Commands, query: Query<(Entity, &Player, &VisualEffect, &Position)>, players_query: Query<(&Position, &SocketWriter)>) {
    for (entity, player, visual_effect, position) in &query {
        let visual_effect_response = VisualEffectResponse::new(player.id, TargetType::Player, &visual_effect.visual_effect);
        for (other_position, other_socket_writer) in &players_query {
            if other_position.is_in_sight(position) {
                other_socket_writer.write(&mut (&visual_effect_response).into());
            }
        }
        commands.entity(entity).remove::<VisualEffect>();
    }
}