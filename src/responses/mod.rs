use crate::framework::packet::Packet;

pub mod server_selected;
pub mod analyze;
pub mod check_hash;
pub mod authentication_error;
pub mod list_player_characters;
pub mod list_player_deleted_characters;
pub mod player_position;
pub mod player_appear;
pub mod player_skills;
pub mod inventory;
pub mod player_information;
pub mod player_extra_health;
pub mod player_extra_strength;
pub mod player_extra_intelligence;
pub mod player_extra_wisdom;
pub mod player_extra_agility;
pub mod guild_members;
pub mod skill_prepare;
pub mod skill_execute;
pub mod chat_message;
pub mod player_walk;
pub mod player_stop_walking;
pub mod emote;
pub mod system_message;
pub mod popup_message;
pub mod character_creation_error;
pub mod character_restoration_error;
pub mod equip_item;
pub mod unequip_item;

#[derive(Debug)]
pub enum ServerPacket {
    ServerSelected(self::server_selected::ServerSelectedResponse),
    Analyze(self::analyze::AnalyzeResponse),
    CheckHash(self::check_hash::CheckHashResponse),
    AuthenticationError(self::authentication_error::AuthenticationErrorResponse),
    ListPlayerCharacters(self::list_player_characters::ListPlayerCharactersResponse),
    ListPlayerDeletedCharacters(self::list_player_deleted_characters::ListPlayerDeletedCharactersResponse),
    PlayerPosition(self::player_position::PlayerPositionResponse),
    PlayerAppear(self::player_appear::PlayerAppearResponse),
    PlayerSkills(self::player_skills::PlayerSkillsResponse),
    Inventory(self::inventory::InventoryResponse),
    PlayerInformation(self::player_information::PlayerInformationResponse),
    PlayerExtraHealth(self::player_extra_health::PlayerExtraHealthResponse),
    PlayerExtraStrength(self::player_extra_strength::PlayerExtraStrengthResponse),
    PlayerExtraIntelligence(self::player_extra_intelligence::PlayerExtraIntelligenceResponse),
    PlayerExtraWisdom(self::player_extra_wisdom::PlayerExtraWisdomResponse),
    PlayerExtraAgility(self::player_extra_agility::PlayerExtraAgilityResponse),
    GuildMembers(self::guild_members::GuildMembersResponse),
    SkillPrepare(self::skill_prepare::SkillPrepareResponse),
    SkillExecute(self::skill_execute::SkillExecuteResponse),
    ChatMessage(self::chat_message::ChatMessageResponse),
    PlayerWalk(self::player_walk::PlayerWalkResponse),
    PlayerStopWalking(self::player_stop_walking::PlayerStopWalkingResponse),
    Emote(self::emote::EmoteResponse),
    SystemMessage(self::system_message::SystemMessageResponse),
    PopupMessage(self::popup_message::PopupMessageResponse),
    CharacterCreationError(self::character_creation_error::CharacterCreationErrorResponse),
    CharacterRestorationError(self::character_restoration_error::CharacterRestorationErrorResponse),
    EquipItem(self::equip_item::EquipItemResponse),
    UnequipItem(self::unequip_item::UnequipItemResponse),
    Unknown(crate::framework::packet::Packet),
}

pub fn deserialize(buffer: &[u8]) -> ServerPacket {
    let mut packet = Packet::new(buffer);
    let header = packet.get_header();
    match header {
        self::server_selected::HEADER => ServerPacket::ServerSelected(self::server_selected::ServerSelectedResponse::from(&mut packet)),
        self::authentication_error::HEADER => ServerPacket::AuthenticationError(self::authentication_error::AuthenticationErrorResponse::from(&mut packet)),
        self::analyze::HEADER => ServerPacket::Analyze(self::analyze::AnalyzeResponse::from(&mut packet)),
        self::list_player_characters::HEADER => ServerPacket::ListPlayerCharacters(self::list_player_characters::ListPlayerCharactersResponse::from(&mut packet)),
        self::list_player_deleted_characters::HEADER => ServerPacket::ListPlayerDeletedCharacters(self::list_player_deleted_characters::ListPlayerDeletedCharactersResponse::from(&mut packet)),
        self::player_position::HEADER => ServerPacket::PlayerPosition(self::player_position::PlayerPositionResponse::from(&mut packet)),
        self::player_appear::HEADER => ServerPacket::PlayerAppear(self::player_appear::PlayerAppearResponse::from(&mut packet)),
        self::player_skills::HEADER => ServerPacket::PlayerSkills(self::player_skills::PlayerSkillsResponse::from(&mut packet)),
        self::inventory::HEADER => ServerPacket::Inventory(self::inventory::InventoryResponse::from(&mut packet)),
        self::player_information::HEADER => ServerPacket::PlayerInformation(self::player_information::PlayerInformationResponse::from(&mut packet)),
        self::skill_prepare::HEADER => ServerPacket::SkillPrepare(self::skill_prepare::SkillPrepareResponse::from(&mut packet)),
        self::skill_execute::HEADER => ServerPacket::SkillExecute(self::skill_execute::SkillExecuteResponse::from(&mut packet)),
        self::chat_message::HEADER => ServerPacket::ChatMessage(self::chat_message::ChatMessageResponse::from(&mut packet)),
        self::player_walk::HEADER => ServerPacket::PlayerWalk(self::player_walk::PlayerWalkResponse::from(&mut packet)),
        self::player_stop_walking::HEADER => ServerPacket::PlayerStopWalking(self::player_stop_walking::PlayerStopWalkingResponse::from(&mut packet)),
        self::emote::HEADER => ServerPacket::Emote(self::emote::EmoteResponse::from(&mut packet)),
        self::character_creation_error::HEADER => ServerPacket::CharacterCreationError(self::character_creation_error::CharacterCreationErrorResponse::from(&mut packet)),
        self::character_restoration_error::HEADER => ServerPacket::CharacterRestorationError(self::character_restoration_error::CharacterRestorationErrorResponse::from(&mut packet)),
        self::equip_item::HEADER => ServerPacket::EquipItem(self::equip_item::EquipItemResponse::from(&mut packet)),
        self::unequip_item::HEADER => ServerPacket::UnequipItem(self::unequip_item::UnequipItemResponse::from(&mut packet)),
        self::check_hash::HEADER => {
            let sub_header = packet.get_u32();
            match sub_header {
                self::check_hash::SUB_HEADER => ServerPacket::CheckHash(self::check_hash::CheckHashResponse::from(&mut packet)),
                self::system_message::SUB_HEADER => ServerPacket::SystemMessage(self::system_message::SystemMessageResponse::from(&mut packet)),
                self::popup_message::SUB_HEADER => ServerPacket::PopupMessage(self::popup_message::PopupMessageResponse::from(&mut packet)),
                _ => ServerPacket::Unknown(packet)
            }
        },
        self::player_extra_health::HEADER => {
            let sub_header = packet.get_u8();
            match sub_header {
                self::player_extra_health::SUB_HEADER => ServerPacket::PlayerExtraHealth(self::player_extra_health::PlayerExtraHealthResponse::from(&mut packet)),
                self::player_extra_strength::SUB_HEADER => ServerPacket::PlayerExtraStrength(self::player_extra_strength::PlayerExtraStrengthResponse::from(&mut packet)),
                self::player_extra_intelligence::SUB_HEADER => ServerPacket::PlayerExtraIntelligence(self::player_extra_intelligence::PlayerExtraIntelligenceResponse::from(&mut packet)),
                self::player_extra_wisdom::SUB_HEADER => ServerPacket::PlayerExtraWisdom(self::player_extra_wisdom::PlayerExtraWisdomResponse::from(&mut packet)),
                self::player_extra_agility::SUB_HEADER => ServerPacket::PlayerExtraAgility(self::player_extra_agility::PlayerExtraAgilityResponse::from(&mut packet)),
                _ => ServerPacket::Unknown(packet)
            }
        },
        self::guild_members::HEADER => {
            let sub_header = packet.get_u8();
            match sub_header {
                self::guild_members::SUB_HEADER => ServerPacket::GuildMembers(self::guild_members::GuildMembersResponse::from(&mut packet)),
                _ => ServerPacket::Unknown(packet)
            }
        },
        _ => ServerPacket::Unknown(packet)
    }
}