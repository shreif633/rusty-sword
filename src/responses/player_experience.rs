use crate::framework::packet::Packet;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 25;

#[derive(Debug)]
pub struct PlayerExperienceResponse {
    pub current_experience: i64,
    pub added_experience: i64,
}


impl From<&mut Packet> for PlayerExperienceResponse {
    fn from(packet: &mut Packet) -> Self {
        let current_experience = packet.get_i64();
        let added_experience = packet.get_i64();
        PlayerExperienceResponse { current_experience, added_experience  }
    }
}

impl From<&PlayerExperienceResponse> for Packet {
    fn from(val: &PlayerExperienceResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_i64(val.current_experience);
        packet.write_i64(val.added_experience);
        packet
    }
}