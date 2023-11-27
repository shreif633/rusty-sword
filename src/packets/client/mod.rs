use crate::framework::packet::Packet;
pub mod server_select;
pub mod authenticate;
pub mod select_character;
pub mod skill_prepare;
pub mod chat_message;
pub mod player_walk;
pub mod player_stop_walking;
pub mod emote;
pub mod create_character;
pub mod delete_character;
pub mod restore_deleted_character;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum ClientPacket {
    ServerSelect(self::server_select::ServerSelect),
    Authenticate(self::authenticate::Authenticate),
    SelectCharacter(self::select_character::SelectCharacter),
    SkillPrepare(self::skill_prepare::SkillPrepare),
    ChatMessage(self::chat_message::ChatMessage),
    PlayerWalk(self::player_walk::PlayerWalk),
    PlayerStopWalking(self::player_stop_walking::PlayerStopWalking),
    Emote(self::emote::Emote),
    CreateCharacter(self::create_character::CreateCharacter),
    DeleteCharacter(self::delete_character::DeleteCharacter),
    RestoreDeletedCharacter(self::restore_deleted_character::RestoreDeletedCharacter),
    Unknown(crate::framework::packet::Packet),
}

pub fn deserialize(buffer: &[u8]) -> ClientPacket {
    let mut packet = Packet::new(buffer);
    let header = packet.get_header();
    match header {
        self::server_select::HEADER => ClientPacket::ServerSelect(self::server_select::ServerSelect::from(&mut packet)),
        self::authenticate::HEADER => ClientPacket::Authenticate(self::authenticate::Authenticate::from(&mut packet)),
        self::select_character::HEADER => ClientPacket::SelectCharacter(self::select_character::SelectCharacter::from(&mut packet)),
        self::skill_prepare::HEADER => ClientPacket::SkillPrepare(self::skill_prepare::SkillPrepare::from(&mut packet)),
        self::chat_message::HEADER => ClientPacket::ChatMessage(self::chat_message::ChatMessage::from(&mut packet)),
        self::player_walk::HEADER => ClientPacket::PlayerWalk(self::player_walk::PlayerWalk::from(&mut packet)),
        self::player_stop_walking::HEADER => ClientPacket::PlayerStopWalking(self::player_stop_walking::PlayerStopWalking::from(&mut packet)),
        self::create_character::HEADER => ClientPacket::CreateCharacter(self::create_character::CreateCharacter::from(&mut packet)),
        self::delete_character::HEADER => ClientPacket::DeleteCharacter(self::delete_character::DeleteCharacter::from(&mut packet)),
        self::restore_deleted_character::HEADER => ClientPacket::RestoreDeletedCharacter(self::restore_deleted_character::RestoreDeletedCharacter::from(&mut packet)),
        self::emote::HEADER => ClientPacket::Emote(self::emote::Emote::from(&mut packet)),
        _ => ClientPacket::Unknown(packet)
    }
}