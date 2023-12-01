use crate::framework::packet::Packet;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 46;

#[derive(Debug)]
pub struct PlayerExtraWisdomResponse {
    pub extra_wisdom: u16,
    pub current_magic_points: u16,
    pub maximum_magic_points: u16,
    pub minimum_magical_attack: u16,
    pub maximum_magical_attack: u16,
    pub curse_resistence: u16,
}

impl From<&mut Packet> for PlayerExtraWisdomResponse {
    fn from(packet: &mut Packet) -> Self {
        let extra_wisdom = packet.get_u16();
        let current_magic_points = packet.get_u16();
        let maximum_magic_points = packet.get_u16();
        let minimum_magical_attack = packet.get_u16();
        let maximum_magical_attack = packet.get_u16();
        let curse_resistence = packet.get_u16();
        PlayerExtraWisdomResponse { extra_wisdom, current_magic_points, maximum_magic_points, minimum_magical_attack, maximum_magical_attack, curse_resistence  }
    }
}

impl From<&PlayerExtraWisdomResponse> for Packet {
    fn from(val: &PlayerExtraWisdomResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_u16(val.extra_wisdom);
        packet.write_u16(val.current_magic_points);
        packet.write_u16(val.maximum_magic_points);
        packet.write_u16(val.minimum_magical_attack);
        packet.write_u16(val.maximum_magical_attack);
        packet.write_u16(val.curse_resistence);
        packet
    }
}