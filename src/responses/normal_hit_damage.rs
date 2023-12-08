use crate::enums::damage_type::DamageType;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 62;

#[derive(Debug)]
pub struct NormalHitDamageResponse {
    pub attacker_id: i32,
    pub target_id: i32,
    pub normal_damage: u32,
    pub explosive_blow_damage: u32,
    pub damage_type: DamageType,
    pub soul_pocket_damage: u32,
}

impl From<&mut Packet> for NormalHitDamageResponse {
    fn from(packet: &mut Packet) -> Self {
        let monster_id = packet.get_i32();
        let target_id = packet.get_i32();
        let normal_damage = packet.get_u32();
        let explosive_blow_damage = packet.get_u32();
        let damage_type = DamageType::from(packet.get_u8());
        let soul_pocket_damage = packet.get_u32();
        NormalHitDamageResponse { attacker_id: monster_id, target_id, normal_damage, explosive_blow_damage, damage_type, soul_pocket_damage }
    }
}

impl From<&NormalHitDamageResponse> for Packet {
    fn from(val: &NormalHitDamageResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.attacker_id);
        packet.write_i32(val.target_id);
        packet.write_u32(val.normal_damage);
        packet.write_u32(val.explosive_blow_damage);
        packet.write_u8(u8::from(val.damage_type));
        packet.write_u32(val.soul_pocket_damage);
        packet
    }
}