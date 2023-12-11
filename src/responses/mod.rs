use crate::framework::packet::Packet;

pub mod server_selected;
pub mod analyze;
pub mod check_hash;
pub mod authentication_error;
pub mod list_player_characters;
pub mod list_player_deleted_characters;
pub mod monster_appear;
pub mod monster_disappear;
pub mod player_experience;
pub mod normal_hit_damage;
pub mod npc_appear;
pub mod player_position;
pub mod player_appear;
pub mod player_skills;
pub mod inventory;
pub mod player_information;
pub mod player_level;
pub mod player_extra_health;
pub mod player_extra_strength;
pub mod player_extra_intelligence;
pub mod player_extra_wisdom;
pub mod player_extra_agility;
pub mod general_state;
pub mod guild_members;
pub mod animation;
pub mod skill_animation;
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
pub mod player_current_health_points;
pub mod visual_effect;
pub mod update_item_quantity;
pub mod player_disappear;
pub mod player_current_magic_points;

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
    SkillPrepare(self::animation::AnimationResponse),
    SkillExecute(self::skill_animation::SkillAnimationResponse),
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
    PlayerCurrentHealthPoints(self::player_current_health_points::PlayerCurrentHealthPointsResponse),
    VisualEffect(self::visual_effect::VisualEffectResponse),
    UpdateItemQuantity(self::update_item_quantity::UpdateItemQuantityResponse),
    MonsterAppear(self::monster_appear::MonsterAppearResponse),
    MonsterDisappear(self::monster_disappear::MonsterDisappearResponse),
    NpcAppear(self::npc_appear::NpcAppearResponse),
    NormalHitDamage(self::normal_hit_damage::NormalHitDamageResponse),
    GeneralState(self::general_state::GeneralStateResponse),
    PlayerDisappear(self::player_disappear::PlayerDisappearResponse),
    PlayerCurrentMagicPoints(self::player_current_magic_points::PlayerCurrentMagicPointsResponse),
    PlayerExperience(self::player_experience::PlayerExperienceResponse),
    PlayerLevel(self::player_level::PlayerLevelResponse),
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
        self::animation::HEADER => ServerPacket::SkillPrepare(self::animation::AnimationResponse::from(&mut packet)),
        self::skill_animation::HEADER => ServerPacket::SkillExecute(self::skill_animation::SkillAnimationResponse::from(&mut packet)),
        self::chat_message::HEADER => ServerPacket::ChatMessage(self::chat_message::ChatMessageResponse::from(&mut packet)),
        self::player_walk::HEADER => ServerPacket::PlayerWalk(self::player_walk::PlayerWalkResponse::from(&mut packet)),
        self::player_stop_walking::HEADER => ServerPacket::PlayerStopWalking(self::player_stop_walking::PlayerStopWalkingResponse::from(&mut packet)),
        self::emote::HEADER => ServerPacket::Emote(self::emote::EmoteResponse::from(&mut packet)),
        self::character_creation_error::HEADER => ServerPacket::CharacterCreationError(self::character_creation_error::CharacterCreationErrorResponse::from(&mut packet)),
        self::character_restoration_error::HEADER => ServerPacket::CharacterRestorationError(self::character_restoration_error::CharacterRestorationErrorResponse::from(&mut packet)),
        self::equip_item::HEADER => ServerPacket::EquipItem(self::equip_item::EquipItemResponse::from(&mut packet)),
        self::unequip_item::HEADER => ServerPacket::UnequipItem(self::unequip_item::UnequipItemResponse::from(&mut packet)),
        self::update_item_quantity::HEADER => ServerPacket::UpdateItemQuantity(self::update_item_quantity::UpdateItemQuantityResponse::from(&mut packet)),
        self::monster_appear::HEADER => ServerPacket::MonsterAppear(self::monster_appear::MonsterAppearResponse::from(&mut packet)),
        self::monster_disappear::HEADER => ServerPacket::MonsterDisappear(self::monster_disappear::MonsterDisappearResponse::from(&mut packet)),
        self::npc_appear::HEADER => ServerPacket::NpcAppear(self::npc_appear::NpcAppearResponse::from(&mut packet)),
        self::normal_hit_damage::HEADER => ServerPacket::NormalHitDamage(self::normal_hit_damage::NormalHitDamageResponse::from(&mut packet)),
        self::general_state::HEADER => ServerPacket::GeneralState(self::general_state::GeneralStateResponse::from(&mut packet)),
        self::player_disappear::HEADER => ServerPacket::PlayerDisappear(self::player_disappear::PlayerDisappearResponse::from(&mut packet)),
        self::check_hash::HEADER => {
            let sub_header = packet.get_u32();
            match sub_header {
                self::check_hash::SUB_HEADER => ServerPacket::CheckHash(self::check_hash::CheckHashResponse::from(&mut packet)),
                self::system_message::SUB_HEADER => ServerPacket::SystemMessage(self::system_message::SystemMessageResponse::from(&mut packet)),
                self::popup_message::SUB_HEADER => ServerPacket::PopupMessage(self::popup_message::PopupMessageResponse::from(&mut packet)),
                self::visual_effect::SUB_HEADER => ServerPacket::VisualEffect(self::visual_effect::VisualEffectResponse::from(&mut packet)),
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
                self::player_current_health_points::SUB_HEADER => ServerPacket::PlayerCurrentHealthPoints(self::player_current_health_points::PlayerCurrentHealthPointsResponse::from(&mut packet)),
                self::player_current_magic_points::SUB_HEADER => ServerPacket::PlayerCurrentMagicPoints(self::player_current_magic_points::PlayerCurrentMagicPointsResponse::from(&mut packet)),
                self::player_experience::SUB_HEADER => ServerPacket::PlayerExperience(self::player_experience::PlayerExperienceResponse::from(&mut packet)),
                self::player_level::SUB_HEADER => ServerPacket::PlayerLevel(self::player_level::PlayerLevelResponse::from(&mut packet)),
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