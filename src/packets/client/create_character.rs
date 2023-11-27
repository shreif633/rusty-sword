use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PlayerClass {
    Knight = 0,
    Mage = 1,
    Archer = 2,
}

#[derive(Component, Debug, Clone)]
pub struct CreateCharacter {
    pub name: String,
    pub class: PlayerClass,
    pub base_strength: u8,
    pub base_health: u8,
    pub base_intelligence: u8,
    pub base_wisdom: u8,
    pub base_agility: u8,
    pub face: u8,
    pub hair: u8,
}

impl From<&mut Packet> for CreateCharacter {
    fn from(packet: &mut Packet) -> Self {
        let name = packet.get_string();
        let class = packet.get_u8();
        let class = match class {
            0 => PlayerClass::Knight,
            1 => PlayerClass::Mage,
            _ => PlayerClass::Archer,
        };
        let base_strength = packet.get_u8();
        let base_health = packet.get_u8();
        let base_intelligence = packet.get_u8();
        let base_wisdom = packet.get_u8();
        let base_agility = packet.get_u8();
        let face = packet.get_u8();
        let hair = packet.get_u8();
        CreateCharacter { name, class, base_strength, base_health, base_intelligence, base_wisdom, base_agility, face, hair }
    }
}