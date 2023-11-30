use crate::framework::packet::Packet;

pub const HEADER: u8 = 255;
pub const SUB_HEADER: u32 = 249;

#[derive(Debug)]
pub struct PopupMessage {
    pub message: String,
}

impl From<&mut Packet> for PopupMessage {
    fn from(packet: &mut Packet) -> Self {
        let message = packet.get_string();
        PopupMessage { message }
    }
}

impl From<&PopupMessage> for Packet {
    fn from(val: &PopupMessage) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(SUB_HEADER);
        packet.write_string(&val.message);
        packet
    }
}