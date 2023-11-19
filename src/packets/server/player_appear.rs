use crate::framework::packet::Packet;

pub const HEADER: u8 = 50;

#[derive(Debug)]
#[repr(u8)]
pub enum PlayerClass {
    Knight = 128,
    Mage = 129,
    Archer = 130,
}

#[derive(Debug)]
pub struct PlayerAppear {
    pub player_id: u32,
    pub name: String,
    pub class: PlayerClass,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub unknown1: Vec<u8>,
    pub weapon_index: u16,
    pub shield_index: u16,
    pub helmet_index: u16,
    pub chest_index: u16,
    pub shorts_index: u16,
    pub gloves_index: u16,
    pub boots_index: u16,
    pub unknown2: Vec<u8>,
    pub face: u8, 
    pub hair: u8, 
    pub unknown3: Vec<u8>,
}

impl From<&mut Packet> for PlayerAppear {
    fn from(packet: &mut Packet) -> Self {
        let player_id = packet.get_u32();
        let name = packet.get_string();
        let class = packet.get_u8();
        let class = match class {
            128 => PlayerClass::Knight,
            129 => PlayerClass::Mage,
            _ => PlayerClass::Archer,
        };
        let x = packet.get_u32();
        let y = packet.get_u32();
        let z = packet.get_u32();
        let unknown1 = packet.get_buffer(10);
        let weapon_index = packet.get_u16();
        let shield_index = packet.get_u16();
        let helmet_index = packet.get_u16();
        let chest_index = packet.get_u16();
        let shorts_index = packet.get_u16();
        let gloves_index = packet.get_u16();
        let boots_index = packet.get_u16();
        let unknown2 = packet.get_buffer(62);
        let face = packet.get_u8();
        let hair = packet.get_u8();
        let unknown3 = packet.get_buffer(54);
        PlayerAppear { player_id, name, x, y, z, unknown1, helmet_index, chest_index, shorts_index, gloves_index, boots_index, unknown2, face, hair, unknown3, weapon_index, shield_index, class }
    }
}

impl From<&PlayerAppear> for Packet {
    fn from(val: &PlayerAppear) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(val.player_id);
        packet.write_string(&val.name);
        match val.class {
            PlayerClass::Knight => packet.write_u8(128),
            PlayerClass::Mage => packet.write_u8(129),
            PlayerClass::Archer => packet.write_u8(130),
        };
        packet.write_u32(val.x);
        packet.write_u32(val.y);
        packet.write_u32(val.z);
        packet.write_buffer(&val.unknown1);
        packet.write_u16(val.weapon_index);
        packet.write_u16(val.shield_index);
        packet.write_u16(val.helmet_index);
        packet.write_u16(val.chest_index);
        packet.write_u16(val.shorts_index);
        packet.write_u16(val.gloves_index);
        packet.write_u16(val.boots_index);
        packet.write_buffer(&val.unknown2);
        packet.write_u8(val.face);
        packet.write_u8(val.hair);
        packet.write_buffer(&val.unknown3);
        packet
    }
}