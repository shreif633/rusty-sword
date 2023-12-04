use crate::framework::packet::Packet;

pub const HEADER: u8 = 5;

#[derive(Debug)]
pub struct EquipItemResponse {
    pub player_id: i32,
    pub item_id: i32,
    pub item_index: u16,
}

impl From<&mut Packet> for EquipItemResponse {
    fn from(packet: &mut Packet) -> Self {
        let player_id = packet.get_i32();
        let item_id = packet.get_i32();
        let item_index = packet.get_u16();
        EquipItemResponse { player_id, item_id, item_index }
    }
}

impl From<&EquipItemResponse> for Packet {
    fn from(val: &EquipItemResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_i32(val.player_id);
        packet.write_i32(val.item_id);
        packet.write_u16(val.item_index);
        packet
    }
}