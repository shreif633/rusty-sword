use crate::framework::packet::Packet;

pub const HEADER: u8 = 43;

#[derive(Debug)]
#[repr(u8)]
pub enum Error {
    Unknown = 0,
    WrongPassword = 2,
    AlreadyLogged = 4,
}

#[derive(Debug)]
pub struct AuthenticationError {
    pub error: Error
}

impl From<&mut Packet> for AuthenticationError {
    fn from(packet: &mut Packet) -> Self {
        let error = packet.get_u8();
        let error = match error {
            2 => Error::WrongPassword,
            4 => Error::AlreadyLogged,
            _ => Error::Unknown
        };
        AuthenticationError { error }
    }
}

impl From<&AuthenticationError> for Packet {
    fn from(val: &AuthenticationError) -> Self {
        let mut packet = Packet::from(HEADER);
        match val.error {
            Error::WrongPassword => packet.write_u8(2),
            Error::AlreadyLogged => packet.write_u8(4),
            Error::Unknown => packet.write_u8(0),
        };
        packet
    }
}