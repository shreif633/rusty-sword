use crate::framework::packet::Packet;

pub const HEADER: u8 = 26;

#[derive(Debug)]
#[repr(u8)]
pub enum Error {
    Unknown = 0,
    SlotLimit = 1,
}

#[derive(Debug)]
pub struct CharacterRestorationErrorResponse {
    pub error: Error
}

impl From<&mut Packet> for CharacterRestorationErrorResponse {
    fn from(packet: &mut Packet) -> Self {
        let error = packet.get_u8();
        let error = match error {
            1 => Error::SlotLimit,
            _ => Error::Unknown
        };
        CharacterRestorationErrorResponse { error }
    }
}

impl From<&CharacterRestorationErrorResponse> for Packet {
    fn from(val: &CharacterRestorationErrorResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        match val.error {
            Error::SlotLimit => packet.write_u8(1),
            Error::Unknown => packet.write_u8(0),
        };
        packet
    }
}