use crate::framework::packet::Packet;
use crate::components::physical_attack::PhysicalAttack;
use crate::components::final_points::FinalPoints;
use crate::components::extra_points::ExtraPoints;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 43;

#[derive(Debug)]
pub struct PlayerExtraStrengthResponse {
    pub extra_strength: u16,
    pub on_target_point: u16,
    pub minimum_physical_attack: u16,
    pub maximum_physical_attack: u16,
}

impl PlayerExtraStrengthResponse {
    pub fn new(extra_points: &ExtraPoints, final_points: &FinalPoints, physical_attack: &PhysicalAttack) -> Self {
        PlayerExtraStrengthResponse {
            extra_strength: extra_points.extra_strength, 
            on_target_point: final_points.on_target_point, 
            minimum_physical_attack: physical_attack.minimum_physical_attack, 
            maximum_physical_attack: physical_attack.maximum_physical_attack 
        }
    }
}

impl From<&mut Packet> for PlayerExtraStrengthResponse {
    fn from(packet: &mut Packet) -> Self {
        let extra_strength = packet.get_u16();
        let on_target_point = packet.get_u16();
        let minimum_physical_attack = packet.get_u16();
        let maximum_physical_attack = packet.get_u16();
        PlayerExtraStrengthResponse { extra_strength, on_target_point, minimum_physical_attack, maximum_physical_attack }
    }
}

impl From<&PlayerExtraStrengthResponse> for Packet {
    fn from(val: &PlayerExtraStrengthResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_u16(val.extra_strength);
        packet.write_u16(val.on_target_point);
        packet.write_u16(val.minimum_physical_attack);
        packet.write_u16(val.maximum_physical_attack);
        packet
    }
}