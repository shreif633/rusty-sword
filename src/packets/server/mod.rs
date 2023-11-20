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

#[derive(Debug)]
pub enum ServerPacket {
    ServerSelected(self::server_selected::ServerSelected),
    Analyze(self::analyze::Analyze),
    CheckHash(self::check_hash::CheckHash),
    AuthenticationError(self::authentication_error::AuthenticationError),
    ListPlayerCharacters(self::list_player_characters::ListPlayerCharacters),
    ListPlayerDeletedCharacters(self::list_player_deleted_characters::ListPlayerDeletedCharacters),
    PlayerPosition(self::player_position::PlayerPosition),
    PlayerAppear(self::player_appear::PlayerAppear),
    PlayerSkills(self::player_skills::PlayerSkills),
    Inventory(self::inventory::Inventory),
    PlayerInformation(self::player_information::PlayerInformation),
    PlayerExtraHealth(self::player_extra_health::PlayerExtraHealth),
    PlayerExtraStrength(self::player_extra_strength::PlayerExtraStrength),
    PlayerExtraIntelligence(self::player_extra_intelligence::PlayerExtraIntelligence),
    PlayerExtraWisdom(self::player_extra_wisdom::PlayerExtraWisdom),
    PlayerExtraAgility(self::player_extra_agility::PlayerExtraAgility),
    GuildMembers(self::guild_members::GuildMembers),
    SkillPrepare(self::skill_prepare::SkillPrepare),
    SkillExecute(self::skill_execute::SkillExecute),
    ChatMessage(self::chat_message::ChatMessage),
    PlayerWalk(self::player_walk::PlayerWalk),
    PlayerStopWalking(self::player_stop_walking::PlayerStopWalking),
    Emote(self::emote::Emote),
    Unknown(crate::framework::packet::Packet),
}

pub fn deserialize(buffer: &[u8]) -> ServerPacket {
    let mut packet = Packet::new(buffer);
    let header = packet.get_header();
    match header {
        self::server_selected::HEADER => ServerPacket::ServerSelected(self::server_selected::ServerSelected::from(&mut packet)),
        self::authentication_error::HEADER => ServerPacket::AuthenticationError(self::authentication_error::AuthenticationError::from(&mut packet)),
        self::analyze::HEADER => ServerPacket::Analyze(self::analyze::Analyze::from(&mut packet)),
        self::list_player_characters::HEADER => ServerPacket::ListPlayerCharacters(self::list_player_characters::ListPlayerCharacters::from(&mut packet)),
        self::list_player_deleted_characters::HEADER => ServerPacket::ListPlayerDeletedCharacters(self::list_player_deleted_characters::ListPlayerDeletedCharacters::from(&mut packet)),
        self::player_position::HEADER => ServerPacket::PlayerPosition(self::player_position::PlayerPosition::from(&mut packet)),
        self::player_appear::HEADER => ServerPacket::PlayerAppear(self::player_appear::PlayerAppear::from(&mut packet)),
        self::player_skills::HEADER => ServerPacket::PlayerSkills(self::player_skills::PlayerSkills::from(&mut packet)),
        self::inventory::HEADER => ServerPacket::Inventory(self::inventory::Inventory::from(&mut packet)),
        self::player_information::HEADER => ServerPacket::PlayerInformation(self::player_information::PlayerInformation::from(&mut packet)),
        self::skill_prepare::HEADER => ServerPacket::SkillPrepare(self::skill_prepare::SkillPrepare::from(&mut packet)),
        self::skill_execute::HEADER => ServerPacket::SkillExecute(self::skill_execute::SkillExecute::from(&mut packet)),
        self::chat_message::HEADER => ServerPacket::ChatMessage(self::chat_message::ChatMessage::from(&mut packet)),
        self::player_walk::HEADER => ServerPacket::PlayerWalk(self::player_walk::PlayerWalk::from(&mut packet)),
        self::player_stop_walking::HEADER => ServerPacket::PlayerStopWalking(self::player_stop_walking::PlayerStopWalking::from(&mut packet)),
        self::emote::HEADER => ServerPacket::Emote(self::emote::Emote::from(&mut packet)),
        self::check_hash::HEADER => {
            let sub_header = packet.get_u32();
            match sub_header {
                self::check_hash::SUB_HEADER => ServerPacket::CheckHash(self::check_hash::CheckHash::from(&mut packet)),
                _ => ServerPacket::Unknown(packet)
            }
        },
        self::player_extra_health::HEADER => {
            let sub_header = packet.get_u8();
            match sub_header {
                self::player_extra_health::SUB_HEADER => ServerPacket::PlayerExtraHealth(self::player_extra_health::PlayerExtraHealth::from(&mut packet)),
                self::player_extra_strength::SUB_HEADER => ServerPacket::PlayerExtraStrength(self::player_extra_strength::PlayerExtraStrength::from(&mut packet)),
                self::player_extra_intelligence::SUB_HEADER => ServerPacket::PlayerExtraIntelligence(self::player_extra_intelligence::PlayerExtraIntelligence::from(&mut packet)),
                self::player_extra_wisdom::SUB_HEADER => ServerPacket::PlayerExtraWisdom(self::player_extra_wisdom::PlayerExtraWisdom::from(&mut packet)),
                self::player_extra_agility::SUB_HEADER => ServerPacket::PlayerExtraAgility(self::player_extra_agility::PlayerExtraAgility::from(&mut packet)),
                _ => ServerPacket::Unknown(packet)
            }
        },
        self::guild_members::HEADER => {
            let sub_header = packet.get_u8();
            match sub_header {
                self::guild_members::SUB_HEADER => ServerPacket::GuildMembers(self::guild_members::GuildMembers::from(&mut packet)),
                _ => ServerPacket::Unknown(packet)
            }
        }
        _ => ServerPacket::Unknown(packet)
    }
}