use crate::{framework::packet::Packet, enums::target_type::TargetType};

pub const HEADER: u8 = 255;
pub const SUB_HEADER: u32 = 239;

#[derive(Debug)]
pub struct VisualEffectResponse {
    pub target_id: u32,
    pub effect_name: String,
    pub remove_automatically: u32,
    pub continuous: u32,
    pub force: u32,
    pub target_type: TargetType
}

impl VisualEffectResponse {
    pub fn new(target_id: u32, target_type: TargetType, effect_name: &str) -> Self {
        VisualEffectResponse { 
            target_id, 
            effect_name: effect_name.to_string(), 
            remove_automatically: 1, 
            continuous: 0, 
            force: 1, 
            target_type 
        }
    }
}

impl From<&mut Packet> for VisualEffectResponse {
    fn from(packet: &mut Packet) -> Self {
        let target_id = packet.get_u32();
        let effect_name = packet.get_string();
        let remove_automatically = packet.get_u32();
        let continuous = packet.get_u32();
        let force = packet.get_u32();
        let target_type = TargetType::from(packet.get_u32() as u8);
        VisualEffectResponse { target_id, effect_name, remove_automatically, continuous, force, target_type  }
    }
}

impl From<&VisualEffectResponse> for Packet {
    fn from(val: &VisualEffectResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(SUB_HEADER);
        packet.write_u32(val.target_id);
        packet.write_string(&val.effect_name);
        packet.write_u32(val.remove_automatically);
        packet.write_u32(val.continuous);
        packet.write_u32(val.force);
        packet.write_u32(u8::from(val.target_type) as u32);
        packet
    }
}