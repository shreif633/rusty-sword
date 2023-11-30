use bevy::prelude::*;
use crate::repositories::item::ItemRow;

#[derive(Component, Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub index: u16,
    pub prefix: u8,
    pub quantity: u32,
    pub maximum_endurance: u8,
    pub current_endurance: u8,
    pub physical_attack_talisman: u8,
    pub magical_attack_talisman: u8,
    pub talisman_of_accuracy: u8,
    pub talisman_of_defence: u8,
    pub upgrade_level: u8,
    pub upgrade_rate: u8
}

impl Item {
    pub fn new(item_row: &ItemRow) -> Self {
        Item { 
            id: item_row.id, 
            index: item_row.index, 
            prefix: item_row.prefix, 
            quantity: item_row.quantity, 
            maximum_endurance: item_row.maximum_endurance, 
            current_endurance: item_row.current_endurance, 
            physical_attack_talisman: item_row.physical_attack_talisman, 
            magical_attack_talisman: item_row.magical_attack_talisman, 
            talisman_of_accuracy: item_row.talisman_of_accuracy, 
            talisman_of_defence: item_row.talisman_of_defence, 
            upgrade_level: item_row.upgrade_level, 
            upgrade_rate: item_row.upgrade_rate 
        }
    }
}
