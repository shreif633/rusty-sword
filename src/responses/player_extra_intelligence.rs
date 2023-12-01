use crate::framework::packet::Packet;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 45;

#[derive(Debug)]
pub struct PlayerExtraIntelligenceResponse {
    pub extra_intelligence: u16,
    pub minimum_magical_attack: u16,
    pub maximum_magical_attack: u16,
    pub fire_resistence: u16,
    pub ice_resistence: u16,
    pub lighning_resistence: u16,
}

impl From<&mut Packet> for PlayerExtraIntelligenceResponse {
    fn from(packet: &mut Packet) -> Self {
        let extra_intelligence = packet.get_u16();
        let minimum_magical_attack = packet.get_u16();
        let maximum_magical_attack = packet.get_u16();
        let fire_resistence = packet.get_u16();
        let ice_resistence = packet.get_u16();
        let lighning_resistence = packet.get_u16();
        PlayerExtraIntelligenceResponse { extra_intelligence, minimum_magical_attack, maximum_magical_attack, fire_resistence, ice_resistence, lighning_resistence }
    }
}

impl From<&PlayerExtraIntelligenceResponse> for Packet {
    fn from(val: &PlayerExtraIntelligenceResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_u16(val.extra_intelligence);
        packet.write_u16(val.minimum_magical_attack);
        packet.write_u16(val.maximum_magical_attack);
        packet.write_u16(val.fire_resistence);
        packet.write_u16(val.ice_resistence);
        packet.write_u16(val.lighning_resistence);
        packet
    }
}