use crate::framework::packet::Packet;

pub const HEADER: u8 = 255;
pub const SUB_HEADER: u32 = 249;

#[derive(Debug)]
pub struct PopupMessageResponse {
    pub message: String,
}

impl From<&mut Packet> for PopupMessageResponse {
    fn from(packet: &mut Packet) -> Self {
        let message = packet.get_string();
        PopupMessageResponse { message }
    }
}

impl From<&PopupMessageResponse> for Packet {
    fn from(val: &PopupMessageResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(SUB_HEADER);
        packet.write_string(&val.message);
        packet
    }
}