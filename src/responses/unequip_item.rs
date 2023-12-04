use crate::framework::packet::Packet;

pub const HEADER: u8 = 6;

#[derive(Debug)]
pub struct UnequipItemResponse {
    pub player_id: i32,
    pub item_id: i32,
    pub item_index: u16,
}

impl From<&mut Packet> for UnequipItemResponse {
    fn from(packet: &mut Packet) -> Self {
        let player_id = packet.get_i32();
        let item_id = packet.get_i32();
        let item_index = packet.get_u16();
        UnequipItemResponse { player_id, item_id, item_index }
    }
}

impl From<&UnequipItemResponse> for Packet {
    fn from(val: &UnequipItemResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.player_id);
        packet.write_i32(val.item_id);
        packet.write_u16(val.item_index);
        packet
    }
}