use bevy::prelude::*;
use text_io::try_read;
use crate::components::admin::Admin;
use crate::components::experience::Experience;
use crate::requests::chat_message::ChatMessageRequest;

pub struct LevelCommandPlugin;

impl Plugin for LevelCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, read_set_level_command);
        app.add_systems(Update, handle_set_level_command);
    }
}

#[derive(Component)]
struct LevelCommand {
    level: u8
}

fn read_set_level_command(mut commands: Commands, query: Query<(Entity, &ChatMessageRequest), With<Admin>>) {
    for (entity, client_packet) in &query {
        if client_packet.message.starts_with("/level") {
            let level: Result<u8, _> = try_read!("/level {}", client_packet.message.bytes());
            if let Ok(level) = level {
                commands.entity(entity).insert(LevelCommand { level });
            }
            commands.entity(entity).remove::<ChatMessageRequest>();
        }
    }
}

fn handle_set_level_command(mut commands: Commands, mut query: Query<(Entity, &LevelCommand, &mut Experience), With<Admin>>) {
    for (entity, command, mut experience) in query.iter_mut() {
        experience.set_to_level(command.level);
        commands.entity(entity).remove::<LevelCommand>();
    }
}