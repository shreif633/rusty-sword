use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 7;

#[derive(Component, Debug, Clone)]
pub struct AuthenticateRequest {
    pub username: String,
    pub password: String,
    pub unknown: String,
}

impl From<&mut Packet> for AuthenticateRequest {
    fn from(packet: &mut Packet) -> Self {
        let username = packet.get_string();
        let password = packet.get_string();
        let unknown = packet.get_string();
        AuthenticateRequest { username, password, unknown }
    }
}