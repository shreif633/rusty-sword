use crate::framework::packet::Packet;

pub const HEADER: u8 = 60;

#[derive(Debug)]
pub struct ChatMessageResponse {
    pub character_name: String,
    pub message: String
}

impl From<&mut Packet> for ChatMessageResponse {
    fn from(packet: &mut Packet) -> Self {
        let character_name = packet.get_string();
        let message = packet.get_string();
        ChatMessageResponse { character_name, message }
    }
}

impl From<&ChatMessageResponse> for Packet {
    fn from(val: &ChatMessageResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_string(&val.character_name);
        packet.write_string(&val.message);
        packet
    }
}