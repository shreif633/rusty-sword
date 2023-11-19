use crate::framework::packet::Packet;

pub const HEADER: u8 = 7;

#[derive(Debug)]
pub struct Authenticate {
    pub username: String,
    pub password: String,
    pub unknown: String,
}

impl From<&mut Packet> for Authenticate {
    fn from(packet: &mut Packet) -> Self {
        let username = packet.get_string();
        let password = packet.get_string();
        let unknown = packet.get_string();
        Authenticate { username, password, unknown }
    }
}