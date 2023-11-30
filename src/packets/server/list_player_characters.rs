use crate::framework::packet::Packet;

pub const HEADER: u8 = 17;

#[derive(Debug)]
pub struct PlayerCharacter {
    pub id: u32,
    pub name: String,
    pub class: u8,
    pub specialty: u8,
    pub level: u8,
    pub unknown1: Vec<u8>,
    pub base_strength: u16,
    pub base_health: u16,
    pub base_intelligence: u16,
    pub base_wisdom: u16,
    pub base_agility: u16,
    pub face: u8,
    pub hair: u8,
    pub items_indexes: Vec<u16>
}

#[derive(Debug)]
pub struct ListPlayerCharacters {
    pub unknown1: Vec<u8>,
    pub characters: Vec<PlayerCharacter>,
}

impl From<&mut Packet> for ListPlayerCharacters {
    fn from(packet: &mut Packet) -> Self {
        let unknown1 = packet.get_buffer(5);
        let characters_count = packet.get_u8();
        let mut characters = Vec::<PlayerCharacter>::with_capacity(characters_count as usize);
        for _ in 0..characters_count {
            let id = packet.get_u32();
            let name = packet.get_string();
            let class = packet.get_u8();
            let specialty = packet.get_u8();
            let level = packet.get_u8();
            let unknown1 = packet.get_buffer(4);
            let base_strength = packet.get_u16();
            let base_health = packet.get_u16();
            let base_intelligence = packet.get_u16();
            let base_wisdom = packet.get_u16();
            let base_agility = packet.get_u16();
            let face = packet.get_u8();
            let hair = packet.get_u8();
            let items_count = packet.get_u8();
            let mut items_indexes = Vec::<u16>::with_capacity(items_count as usize);
            for _ in 0..items_count {
                let item_index = packet.get_u16();
                items_indexes.push(item_index);
            }
            let character = PlayerCharacter { 
                id, name, class, specialty, level, base_strength, base_health, 
                base_intelligence, base_wisdom, base_agility, face, hair, items_indexes,
                unknown1, 
            };
            characters.push(character);
        }
        ListPlayerCharacters { unknown1, characters }
    }
}

impl From<&ListPlayerCharacters> for Packet {
    fn from(val: &ListPlayerCharacters) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_buffer(&val.unknown1);
        packet.write_u8(val.characters.len().try_into().unwrap());
        for character in &val.characters {
            packet.write_u32(character.id);
            packet.write_string(&character.name);
            packet.write_u8(character.class);
            packet.write_u8(character.specialty);
            packet.write_u8(character.level);
            packet.write_buffer(&character.unknown1);
            packet.write_u16(character.base_strength);
            packet.write_u16(character.base_health);
            packet.write_u16(character.base_intelligence);
            packet.write_u16(character.base_wisdom);
            packet.write_u16(character.base_agility);
            packet.write_u8(character.face);
            packet.write_u8(character.hair);
            packet.write_u8(character.items_indexes.len().try_into().unwrap());
            for item_index in &character.items_indexes {
                packet.write_u16(*item_index);
            }
        }
        packet
    }
}