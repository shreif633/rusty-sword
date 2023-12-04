use crate::{framework::packet::Packet, repositories::player::PlayerRow};

pub const HEADER: u8 = 25;

#[derive(Debug)]
pub struct PlayerDeletedCharacter {
    pub id: i32,
    pub name: String,
    pub level: u8,
    pub class: u8,
    pub remaining_days: u8,
}

#[derive(Debug)]
pub struct ListPlayerDeletedCharactersResponse {
    pub characters: Vec<PlayerDeletedCharacter>,
}

impl ListPlayerDeletedCharactersResponse {
    pub fn new(player_rows: &[PlayerRow]) -> Self {
        let characters = player_rows.iter().map(|player_row| {
            PlayerDeletedCharacter { 
                id: player_row.id, 
                name: player_row.name.clone(), 
                level: player_row.level, 
                remaining_days: 8,
                class: player_row.class, 
            }
        }).collect();
        ListPlayerDeletedCharactersResponse { characters }
    }
}

impl From<&mut Packet> for ListPlayerDeletedCharactersResponse {
    fn from(packet: &mut Packet) -> Self {
        let characters_count = packet.get_u8();
        let mut characters = Vec::<PlayerDeletedCharacter>::with_capacity(characters_count as usize);
        for _ in 0..characters_count {
            let id = packet.get_i32();
            let name = packet.get_string();
            let class = packet.get_u8();
            let level = packet.get_u8();
            let remaining_days = packet.get_u8();
            let character = PlayerDeletedCharacter { id, name, level, class, remaining_days };
            characters.push(character);
        }
        ListPlayerDeletedCharactersResponse { characters }
    }
}

impl From<&ListPlayerDeletedCharactersResponse> for Packet {
    fn from(val: &ListPlayerDeletedCharactersResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(val.characters.len().try_into().unwrap());
        for character in &val.characters {
            packet.write_i32(character.id);
            packet.write_string(&character.name);
            packet.write_u8(character.level);
            packet.write_u8(character.class);
            packet.write_u8(character.remaining_days);
        }
        packet
    }
}