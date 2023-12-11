use bevy::prelude::*;
use crate::components::appearance::Appearance;
use crate::components::network_writer::NetworkWriter;
use crate::responses::chat_message::ChatMessageResponse;
use crate::components::position::Position;
use crate::requests::chat_message::ChatMessageRequest;

pub struct ChatPlugin;

impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, handle_chat_message);
    }
}

fn handle_chat_message(mut commands: Commands, query: Query<(Entity, &ChatMessageRequest, &Position, &Appearance)>, players: Query<(&Position, &NetworkWriter)>) {
    for (entity, client_packet, chatting_position, chatting_appearence) in &query {
        let chat_message = ChatMessageResponse { 
            character_name: chatting_appearence.name.clone(), 
            message: client_packet.message.clone()
        };
        for (position, socket_writer) in &players {
            if position.is_in_sight(chatting_position) {
                socket_writer.write(&mut (&chat_message).into());
            }
        }
        commands.entity(entity).remove::<ChatMessageRequest>();
    }
}