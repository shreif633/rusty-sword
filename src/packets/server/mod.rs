use crate::framework::packet::Packet;

pub mod server_selected;
pub mod analyze;
pub mod check_hash;
pub mod authentication_error;
pub mod list_player_characters;
pub mod list_player_deleted_characters;
pub mod player_position;

#[derive(Debug)]
pub enum ServerPacket {
    ServerSelected(self::server_selected::ServerSelected),
    Analyze(self::analyze::Analyze),
    CheckHash(self::check_hash::CheckHash),
    AuthenticationError(self::authentication_error::AuthenticationError),
    ListPlayerCharacters(self::list_player_characters::ListPlayerCharacters),
    ListPlayerDeletedCharacters(self::list_player_deleted_characters::ListPlayerDeletedCharacters),
    PlayerPosition(self::player_position::PlayerPosition),
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
        self::check_hash::HEADER => {
            let sub_header = packet.get_u32();
            match sub_header {
                self::check_hash::SUB_HEADER => ServerPacket::CheckHash(self::check_hash::CheckHash::from(&mut packet)),
                _ => ServerPacket::Unknown(packet)
            }
        }
        _ => ServerPacket::Unknown(packet)
    }
}