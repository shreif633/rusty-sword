use bevy::prelude::*;
use crate::requests::emote::EmoteRequest;
use crate::responses::emote::EmoteResponse;
use super::select_character::Player;
use super::player_movement::{Position, Walking};
use super::tcp_server::SocketWriter;

pub struct EmotePlugin;

impl Plugin for EmotePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_emote);
    }
}

fn handle_emote(mut commands: Commands, emote_query: Query<(Entity, &Player, &EmoteRequest, &Position, Option<&Walking>)>, players_query: Query<(&Position, &SocketWriter)>) {
    for (entity, emote_player, client_packet, emote_position, walking) in &emote_query {
        if walking.is_none() {
            let emote = EmoteResponse { 
                player_id: emote_player.id, 
                emote_index: client_packet.emote_index
            };
            for (position, socket_writer) in &players_query {
                if position.is_in_sight(&emote_position) {
                    socket_writer.write(&mut (&emote).into());
                }
            }
        }
        commands.entity(entity).remove::<EmoteRequest>();
    }
}