use sqlx::query;
use crate::framework::database::Database;

pub struct ItemRow {
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

pub fn find_all_items_by_player_id(database: &Database, player_id: u32) -> Vec<ItemRow> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let rows = rt.block_on(async move {
        query!("SELECT * FROM items WHERE player_id = ?", player_id).fetch_all(&database.connection).await.unwrap()
    });
    let mut items: Vec<ItemRow> = vec![];
    for row in rows {
        let item = ItemRow { 
            id: row.id.try_into().unwrap(), 
            index: row.item_index.try_into().unwrap(), 
            prefix: row.prefix.try_into().unwrap(), 
            quantity: row.quantity.try_into().unwrap(), 
            maximum_endurance: row.maximum_endurance.try_into().unwrap(), 
            current_endurance: row.current_endurance.try_into().unwrap(), 
            physical_attack_talisman: row.physical_attack_talisman.try_into().unwrap(), 
            magical_attack_talisman: row.magical_attack_talisman.try_into().unwrap(), 
            talisman_of_accuracy: row.talisman_of_accuracy.try_into().unwrap(), 
            talisman_of_defence: row.talisman_of_defence.try_into().unwrap(), 
            upgrade_level: row.upgrade_level.try_into().unwrap(), 
            upgrade_rate: row.upgrade_rate.try_into().unwrap(), 
        };
        items.push(item);
    }
    items
}