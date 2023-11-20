use crate::framework::packet::{Packet, HandlePacket};
use crate::framework::world::WorldLock;
pub mod server_select;
pub mod authenticate;
pub mod select_character;
pub mod skill_prepare;

#[derive(Debug)]
pub enum ClientPacket {
    ServerSelect(self::server_select::ServerSelect),
    Authenticate(self::authenticate::Authenticate),
    SelectCharacter(self::select_character::SelectCharacter),
    SkillPrepare(self::skill_prepare::SkillPrepare),
    Unknown(crate::framework::packet::Packet),
}

pub fn deserialize(buffer: &[u8]) -> ClientPacket {
    let mut packet = Packet::new(buffer);
    let header = packet.get_header();
    match header {
        self::server_select::HEADER => ClientPacket::ServerSelect(self::server_select::ServerSelect::from(&mut packet)),
        self::authenticate::HEADER => ClientPacket::Authenticate(self::authenticate::Authenticate::from(&mut packet)),
        self::select_character::HEADER => ClientPacket::SelectCharacter(self::select_character::SelectCharacter::from(&mut packet)),
        self::skill_prepare::HEADER => ClientPacket::SkillPrepare(self::skill_prepare::SkillPrepare::from(&mut packet)),
        _ => ClientPacket::Unknown(packet)
    }
}

impl ClientPacket {
    pub async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        match self {
            ClientPacket::ServerSelect(packet) => packet.handle(world, user_id).await,
            ClientPacket::Authenticate(packet) => packet.handle(world, user_id).await,
            ClientPacket::SelectCharacter(packet) => packet.handle(world, user_id).await,
            ClientPacket::SkillPrepare(packet) => packet.handle(world, user_id).await,
            ClientPacket::Unknown(packet) => packet.handle(user_id).await
        }
    }
}