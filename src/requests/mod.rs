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
pub mod equip_item;
pub mod unequip_item;
pub mod use_item;
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub enum ClientPacket {
    ServerSelect(self::server_select::ServerSelectRequest),
    Authenticate(self::authenticate::AuthenticateRequest),
    SelectCharacter(self::select_character::SelectCharacterRequest),
    SkillPrepare(self::skill_prepare::SkillPrepareRequest),
    ChatMessage(self::chat_message::ChatMessageRequest),
    PlayerWalk(self::player_walk::PlayerWalkRequest),
    PlayerStopWalking(self::player_stop_walking::PlayerStopWalkingRequest),
    Emote(self::emote::EmoteRequest),
    CreateCharacter(self::create_character::CreateCharacterRequest),
    DeleteCharacter(self::delete_character::DeleteCharacterRequest),
    RestoreDeletedCharacter(self::restore_deleted_character::RestoreDeletedCharacterRequest),
    EquipItem(self::equip_item::EquipItemRequest),
    UnequipItem(self::unequip_item::UnequipItemRequest),
    UseItem(self::use_item::UseItemRequest),
    Unknown(crate::framework::packet::Packet),
}

pub fn deserialize(buffer: &[u8]) -> ClientPacket {
    let mut packet = Packet::new(buffer);
    let header = packet.get_header();
    match header {
        self::server_select::HEADER => ClientPacket::ServerSelect(self::server_select::ServerSelectRequest::from(&mut packet)),
        self::authenticate::HEADER => ClientPacket::Authenticate(self::authenticate::AuthenticateRequest::from(&mut packet)),
        self::select_character::HEADER => ClientPacket::SelectCharacter(self::select_character::SelectCharacterRequest::from(&mut packet)),
        self::skill_prepare::HEADER => ClientPacket::SkillPrepare(self::skill_prepare::SkillPrepareRequest::from(&mut packet)),
        self::chat_message::HEADER => ClientPacket::ChatMessage(self::chat_message::ChatMessageRequest::from(&mut packet)),
        self::player_walk::HEADER => ClientPacket::PlayerWalk(self::player_walk::PlayerWalkRequest::from(&mut packet)),
        self::player_stop_walking::HEADER => ClientPacket::PlayerStopWalking(self::player_stop_walking::PlayerStopWalkingRequest::from(&mut packet)),
        self::create_character::HEADER => ClientPacket::CreateCharacter(self::create_character::CreateCharacterRequest::from(&mut packet)),
        self::delete_character::HEADER => ClientPacket::DeleteCharacter(self::delete_character::DeleteCharacterRequest::from(&mut packet)),
        self::restore_deleted_character::HEADER => ClientPacket::RestoreDeletedCharacter(self::restore_deleted_character::RestoreDeletedCharacterRequest::from(&mut packet)),
        self::equip_item::HEADER => ClientPacket::EquipItem(self::equip_item::EquipItemRequest::from(&mut packet)),
        self::unequip_item::HEADER => ClientPacket::UnequipItem(self::unequip_item::UnequipItemRequest::from(&mut packet)),
        self::use_item::HEADER => ClientPacket::UseItem(self::use_item::UseItemRequest::from(&mut packet)),
        self::emote::HEADER => ClientPacket::Emote(self::emote::EmoteRequest::from(&mut packet)),
        _ => ClientPacket::Unknown(packet)
    }
}