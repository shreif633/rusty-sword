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

pub fn find_all_items_by_player_id(database: &Database, player_id: i32) -> Vec<ItemRow> {
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

pub struct ItemCreateChangeset {
    pub player_id: i32,
    pub index: u16,
    pub prefix: u8,
    pub quantity: u32
}

pub fn create_item(database: &Database, changeset: &ItemCreateChangeset) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query!("
        INSERT INTO items 
        (player_id, item_index, prefix, quantity, maximum_endurance, current_endurance, physical_attack_talisman, magical_attack_talisman, talisman_of_accuracy, talisman_of_defence, upgrade_level, upgrade_rate) 
        values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ", changeset.player_id, changeset.index, changeset.prefix, changeset.quantity, 0, 0, 0, 0, 0, 0, 0, 0)
        .execute(&database.connection).await.unwrap();
    });
}

pub struct ItemUpdateQuantityChangeset {
    pub id: i32,
    pub quantity: u32,
}

pub fn update_all_item_quantity_by_id(
    database: &Database,
    changesets: &Vec<ItemUpdateQuantityChangeset>
) {
    let mut update = "".to_string();
    for changeset in changesets {
        update.push_str(&format!("UPDATE items SET quantity = {} WHERE id = {};", changeset.quantity, changeset.id));
    }
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    rt.block_on(async move {
        query(&update).execute(&database.connection).await.unwrap()
    });
} 