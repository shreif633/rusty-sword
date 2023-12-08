use crate::components::id::Id;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 55;

#[derive(Debug)]
pub struct PlayerDisappearResponse {
    pub id: i32,
}

impl PlayerDisappearResponse {
    pub fn new(id: &Id) -> Self {
        PlayerDisappearResponse { 
            id: id.id,
        }
    }
}

impl From<&mut Packet> for PlayerDisappearResponse {
    fn from(packet: &mut Packet) -> Self {
        let id = packet.get_i32();
        PlayerDisappearResponse { id }
    }
}

impl From<&PlayerDisappearResponse> for Packet {
    fn from(val: &PlayerDisappearResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.id);
        packet
    }
}