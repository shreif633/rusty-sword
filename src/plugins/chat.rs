use bevy::prelude::*;
use crate::responses::chat_message::ChatMessageResponse;
use crate::requests::chat_message::ChatMessageRequest;
use super::{tcp_server::SocketWriter, player_movement::Position, select_character::Appearence};

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_chat_message);
    }
}

fn handle_chat_message(mut commands: Commands, emote_query: Query<(Entity, &ChatMessageRequest, &Position, &Appearence)>, players_query: Query<(&Position, &SocketWriter)>) {
    for (entity, client_packet, chatting_position, chatting_appearence) in &emote_query {
        let chat_message = ChatMessageResponse { 
            character_name: chatting_appearence.name.clone(), 
            message: client_packet.message.clone()
        };
        for (position, socket_writer) in &players_query {
            if position.is_in_sight(&chatting_position) {
                socket_writer.write(&mut (&chat_message).into());
            }
        }
        commands.entity(entity).remove::<ChatMessageRequest>();
    }
}