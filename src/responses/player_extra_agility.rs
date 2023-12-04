use crate::framework::packet::Packet;
use crate::components::physical_attack::PhysicalAttack;
use crate::components::final_points::FinalPoints;
use crate::components::extra_points::ExtraPoints;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 47;

#[derive(Debug)]
pub struct PlayerExtraAgilityResponse {
    pub extra_agility: u16,
    pub on_target_point: u16,
    pub evasion: u16,
    pub unknown_evasion_copy: u16,
    pub minimum_physical_attack: u16,
    pub maximum_physical_attack: u16,
}

impl PlayerExtraAgilityResponse {
    pub fn new(extra_points: &ExtraPoints, final_points: &FinalPoints, physical_attack: &PhysicalAttack) -> Self {
        PlayerExtraAgilityResponse {
            extra_agility: extra_points.extra_agility, 
            on_target_point: final_points.on_target_point, 
            evasion: final_points.evasion, 
            unknown_evasion_copy: final_points.evasion, 
            minimum_physical_attack: physical_attack.minimum_physical_attack, 
            maximum_physical_attack: physical_attack.maximum_physical_attack
        }
    }
}

impl From<&mut Packet> for PlayerExtraAgilityResponse {
    fn from(packet: &mut Packet) -> Self {
        let extra_agility = packet.get_u16();
        let on_target_point = packet.get_u16();
        let evasion = packet.get_u16();
        let unknown_evasion_copy = packet.get_u16();
        let minimum_physical_attack = packet.get_u16();
        let maximum_physical_attack = packet.get_u16();
        PlayerExtraAgilityResponse { extra_agility, on_target_point, evasion, unknown_evasion_copy, minimum_physical_attack, maximum_physical_attack  }
    }
}

impl From<&PlayerExtraAgilityResponse> for Packet {
    fn from(val: &PlayerExtraAgilityResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_u16(val.extra_agility);
        packet.write_u16(val.on_target_point);
        packet.write_u16(val.evasion);
        packet.write_u16(val.unknown_evasion_copy);
        packet.write_u16(val.minimum_physical_attack);
        packet.write_u16(val.maximum_physical_attack);
        packet
    }
}