use crate::framework::packet::Packet;

pub const HEADER: u8 = 26;

#[derive(Debug)]
#[repr(u8)]
pub enum Error {
    Unknown = 0,
    SlotLimit = 1,
}

#[derive(Debug)]
pub struct CharacterRestorationError {
    pub error: Error
}

impl From<&mut Packet> for CharacterRestorationError {
    fn from(packet: &mut Packet) -> Self {
        let error = packet.get_u8();
        let error = match error {
            1 => Error::SlotLimit,
            _ => Error::Unknown
        };
        CharacterRestorationError { error }
    }
}

impl From<&CharacterRestorationError> for Packet {
    fn from(val: &CharacterRestorationError) -> Self {
        let mut packet = Packet::from(HEADER);
        match val.error {
            Error::SlotLimit => packet.write_u8(1),
            Error::Unknown => packet.write_u8(0),
        };
        packet
    }
}