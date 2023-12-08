use crate::components::id::Id;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 57;

#[derive(Debug)]
pub struct MonsterDisappearResponse {
    pub id: i32,
}

impl MonsterDisappearResponse {
    pub fn new(id: &Id) -> Self {
        MonsterDisappearResponse {
            id: id.id,
        }
    }
}

impl From<&mut Packet> for MonsterDisappearResponse {
    fn from(packet: &mut Packet) -> Self {
        let id = packet.get_i32();
        MonsterDisappearResponse { id }
    }
}

impl From<&MonsterDisappearResponse> for Packet {
    fn from(val: &MonsterDisappearResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.id);
        packet
    }
}