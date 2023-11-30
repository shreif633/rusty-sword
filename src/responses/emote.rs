use crate::framework::packet::Packet;

pub const HEADER: u8 = 24;

#[derive(Debug)]
pub struct EmoteResponse {
    pub player_id: u32,
    pub emote_index: u8
}

impl From<&mut Packet> for EmoteResponse {
    fn from(packet: &mut Packet) -> Self {
        let player_id = packet.get_u32();
        let emote_index = packet.get_u8();
        EmoteResponse { player_id, emote_index }
    }
}

impl From<&EmoteResponse> for Packet {
    fn from(val: &EmoteResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(val.player_id);
        packet.write_u8(val.emote_index);
        packet
    }
}