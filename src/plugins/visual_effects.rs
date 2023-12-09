use bevy::prelude::*;
use crate::components::id::Id;
use crate::components::observers::Observers;
use crate::responses::visual_effect::VisualEffectResponse;
use crate::enums::target_type::TargetType;
use crate::components::visual_effect::VisualEffect;
use super::tcp_server::SocketWriter;

pub struct VisualEffectPlugin;

impl Plugin for VisualEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, show_visual_effects);
    }
}

fn show_visual_effects(mut commands: Commands, query: Query<(Entity, &Id, &VisualEffect, &Observers)>, observers: Query<&SocketWriter>) {
    for (entity, id, visual_effect, target_observers) in &query {
        let visual_effect_response = VisualEffectResponse::new(id.id, TargetType::Player, &visual_effect.visual_effect);
        for entity in &target_observers.entities {
            if let Ok(observer_socket_writer) = observers.get(*entity) {
                observer_socket_writer.write(&mut (&visual_effect_response).into());
            }
        }
        commands.entity(entity).remove::<VisualEffect>();
    }
}