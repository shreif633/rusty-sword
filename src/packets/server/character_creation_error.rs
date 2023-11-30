use crate::framework::packet::Packet;

pub const HEADER: u8 = 44;

#[derive(Debug)]
#[repr(u8)]
pub enum Error {
    Unknown = 0,
    NameTaken = 4,
}

#[derive(Debug)]
pub struct CharacterCreationError {
    pub error: Error
}

impl From<&mut Packet> for CharacterCreationError {
    fn from(packet: &mut Packet) -> Self {
        let error = packet.get_u8();
        let error = match error {
            4 => Error::NameTaken,
            _ => Error::Unknown
        };
        CharacterCreationError { error }
    }
}

impl From<&CharacterCreationError> for Packet {
    fn from(val: &CharacterCreationError) -> Self {
        let mut packet = Packet::from(HEADER);
        match val.error {
            Error::NameTaken => packet.write_u8(4),
            Error::Unknown => packet.write_u8(0),
        };
        packet
    }
}