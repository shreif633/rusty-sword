use bevy::prelude::*;
use crate::enums::player_class::PlayerClass;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 5;

#[derive(Component, Debug, Clone)]
pub struct CreateCharacterRequest {
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

impl From<&mut Packet> for CreateCharacterRequest {
    fn from(packet: &mut Packet) -> Self {
        let name = packet.get_string();
        let class = PlayerClass::from(packet.get_u8());
        let base_strength = packet.get_u8();
        let base_health = packet.get_u8();
        let base_intelligence = packet.get_u8();
        let base_wisdom = packet.get_u8();
        let base_agility = packet.get_u8();
        let face = packet.get_u8();
        let hair = packet.get_u8();
        CreateCharacterRequest { name, class, base_strength, base_health, base_intelligence, base_wisdom, base_agility, face, hair }
    }
}