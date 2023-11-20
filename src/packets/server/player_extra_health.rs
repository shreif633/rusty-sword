use crate::framework::packet::Packet;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 44;

#[derive(Debug)]
pub struct PlayerExtraHealth {
    pub extra_health: u16,
    pub current_health_points: u32,
    pub maximum_health_points: u32,
    pub non_elemental_resistence: u16,
}

impl From<&mut Packet> for PlayerExtraHealth {
    fn from(packet: &mut Packet) -> Self {
        let extra_health = packet.get_u16();
        let current_health_points = packet.get_u32();
        let maximum_health_points = packet.get_u32();
        let non_elemental_resistence = packet.get_u16();
        PlayerExtraHealth { extra_health, current_health_points, maximum_health_points, non_elemental_resistence  }
    }
}

impl From<&PlayerExtraHealth> for Packet {
    fn from(val: &PlayerExtraHealth) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_u16(val.extra_health);
        packet.write_u32(val.current_health_points);
        packet.write_u32(val.maximum_health_points);
        packet.write_u16(val.non_elemental_resistence);
        packet
    }
}