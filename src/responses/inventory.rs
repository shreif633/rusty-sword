use crate::{framework::packet::Packet, components::item::Item};

pub const HEADER: u8 = 4;

#[derive(Debug)]
pub struct InventoryItem {
    pub index: u16,
    pub id: i32,
    pub prefix: u8,
    pub info: u32,
    pub quantity: u32,
    pub maximum_endurance: u8,
    pub current_endurance: u8,
    pub unknown1: u8,
    pub physical_attack_talisman: u8,
    pub magical_attack_talisman: u8,
    pub unknown2: Vec<u8>,
    pub talisman_of_accuracy: u8,
    pub unknown3: Vec<u8>,
    pub talisman_of_defence: u8,
    pub unknown4: Vec<u8>,
    pub upgrade_level: u8,
    pub upgrade_rate: u8,
    pub seconds_remaining: u32,
    pub unknown5: Vec<u8>,
}

#[derive(Debug)]
pub struct InventoryResponse {
    pub items: Vec<InventoryItem>
}

impl InventoryResponse {
    pub fn new(items: Vec<Item>) -> Self {
        let items = items.iter().map(|item| {
            InventoryItem { 
                index: item.index, 
                id: item.id, 
                prefix: item.prefix, 
                info: 0, 
                quantity: 100,
                maximum_endurance: item.maximum_endurance, 
                current_endurance: item.current_endurance, 
                unknown1: 0, 
                physical_attack_talisman: item.physical_attack_talisman, 
                magical_attack_talisman: item.magical_attack_talisman, 
                unknown2: vec![0], 
                talisman_of_accuracy: item.talisman_of_accuracy, 
                unknown3: vec![0], 
                talisman_of_defence: item.talisman_of_defence, 
                unknown4: vec![57], 
                upgrade_level: item.upgrade_level, 
                upgrade_rate: item.upgrade_rate, 
                seconds_remaining: 0, 
                unknown5: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }
        }).collect();
        InventoryResponse { items }
    }
}

impl From<&mut Packet> for InventoryResponse {
    fn from(packet: &mut Packet) -> Self {
        let items_count = packet.get_u16() as usize;
        let mut items = Vec::<InventoryItem>::with_capacity(items_count);
        for _ in 0..items_count {
            let index = packet.get_u16();
            let id = packet.get_i32();
            let prefix = packet.get_u8();
            let info = packet.get_u32();
            let quantity = packet.get_u32();
            let maximum_endurance = packet.get_u8();
            let current_endurance = packet.get_u8();
            let unknown1 = packet.get_u8();
            let physical_attack_talisman = packet.get_u8();
            let magical_attack_talisman = packet.get_u8();
            let unknown2 = packet.get_buffer(1);
            let talisman_of_accuracy = packet.get_u8();
            let unknown3 = packet.get_buffer(1);
            let talisman_of_defence = packet.get_u8();
            let unknown4 = packet.get_buffer(1);
            let upgrade_level = packet.get_u8();
            let upgrade_rate = packet.get_u8();
            let seconds_remaining = packet.get_u32();
            let unknown5 = packet.get_buffer(36);
            let playground = InventoryItem { 
                index, id, prefix, info, quantity, maximum_endurance, 
                current_endurance, unknown1, physical_attack_talisman, 
                magical_attack_talisman, unknown2, talisman_of_accuracy, 
                unknown3, talisman_of_defence, unknown4, upgrade_level, upgrade_rate, 
                seconds_remaining, unknown5 
            };
            items.push(playground);
        }
        InventoryResponse { items }
    }
}

impl From<&InventoryResponse> for Packet {
    fn from(val: &InventoryResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u16(val.items.len().try_into().unwrap());
        for item in &val.items {
            packet.write_u16(item.index);
            packet.write_i32(item.id);
            packet.write_u8(item.prefix);
            packet.write_u32(item.info);
            packet.write_u32(item.quantity);
            packet.write_u8(item.maximum_endurance);
            packet.write_u8(item.current_endurance);
            packet.write_u8(item.unknown1);
            packet.write_u8(item.physical_attack_talisman);
            packet.write_u8(item.magical_attack_talisman);
            packet.write_buffer(&item.unknown2);
            packet.write_u8(item.talisman_of_accuracy);
            packet.write_buffer(&item.unknown3);
            packet.write_u8(item.talisman_of_defence);
            packet.write_buffer(&item.unknown4);
            packet.write_u8(item.upgrade_level);
            packet.write_u8(item.upgrade_rate);
            packet.write_u32(item.seconds_remaining);
            packet.write_buffer(&item.unknown5);
        }
        packet
    }
}