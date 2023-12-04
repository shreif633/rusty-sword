use bevy::prelude::*;
use crate::components::id::Id;
use crate::responses::visual_effect::VisualEffectResponse;
use crate::enums::target_type::TargetType;
use crate::components::position::Position;
use crate::components::visual_effect::VisualEffect;
use super::tcp_server::SocketWriter;

pub struct VisualEffectPlugin;

impl Plugin for VisualEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, show_visual_effects);
    }
}

fn show_visual_effects(mut commands: Commands, query: Query<(Entity, &Id, &VisualEffect, &Position)>, players_query: Query<(&Position, &SocketWriter)>) {
    for (entity, id, visual_effect, position) in &query {
        let visual_effect_response = VisualEffectResponse::new(id.id, TargetType::Player, &visual_effect.visual_effect);
        for (other_position, other_socket_writer) in &players_query {
            if other_position.is_in_sight(position) {
                other_socket_writer.write(&mut (&visual_effect_response).into());
            }
        }
        commands.entity(entity).remove::<VisualEffect>();
    }
}