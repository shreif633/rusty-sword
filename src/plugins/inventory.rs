use bevy::prelude::*;
use sqlx::query;
use crate::{framework::database::Database, packets::client::{equip_item::EquipItem, unequip_item::UnequipItem, use_item::{UseItem, self}}};
use super::{tcp_server::SocketWriter, select_character::Player, player_movement::Position};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_inventory);
        app.add_systems(Update, equip_item.before(broadcast_weapon_change));
        app.add_systems(Update, unequip_item.before(broadcast_weapon_change));
        app.add_systems(Update, broadcast_weapon_change);
        app.add_systems(Update, use_item);
    }
}

#[derive(Component)]
struct ItemRow {
    id: i32,
    index: u16,
    prefix: u8,
    quantity: u32,
    maximum_endurance: u8,
    current_endurance: u8,
    physical_attack_talisman: u8,
    magical_attack_talisman: u8,
    talisman_of_accuracy: u8,
    talisman_of_defence: u8,
    upgrade_level: u8,
    upgrade_rate: u8
}

#[derive(Component, Debug)]
struct Item {
    id: i32,
    index: u16,
    prefix: u8,
    quantity: u32,
    maximum_endurance: u8,
    current_endurance: u8,
    physical_attack_talisman: u8,
    magical_attack_talisman: u8,
    talisman_of_accuracy: u8,
    talisman_of_defence: u8,
    upgrade_level: u8,
    upgrade_rate: u8
}

#[derive(Component)]
struct PlayerOwner {
    player: Entity
}

fn query_player_items(database: &Database, player_id: u32) -> Vec<ItemRow> {
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

fn load_inventory(mut commands: Commands, query: Query<(Entity, Added<Player>, &Player, &SocketWriter)>, database: Res<Database>) {
    for (entity, added_player, player, socket_writer) in &query {
        if added_player {
            let player_items = query_player_items(&database, player.id);
            let items: Vec<crate::packets::server::inventory::Item> = player_items.iter().map(|i| {
                let item_id = commands.spawn((
                    Item { 
                        id: i.id, 
                        index: i.index, 
                        prefix: i.prefix, 
                        quantity: i.quantity, 
                        maximum_endurance: i.maximum_endurance, 
                        current_endurance: i.current_endurance, 
                        physical_attack_talisman: i.physical_attack_talisman, 
                        magical_attack_talisman: i.magical_attack_talisman, 
                        talisman_of_accuracy: i.talisman_of_accuracy, 
                        talisman_of_defence: i.talisman_of_defence, 
                        upgrade_level: i.upgrade_level, 
                        upgrade_rate: i.upgrade_rate 
                    },
                    PlayerOwner { 
                        player: entity 
                    }
                )).id().index();
                crate::packets::server::inventory::Item { 
                    index: i.index, 
                    id: item_id.try_into().unwrap(), 
                    prefix: i.prefix, 
                    info: 0, 
                    quantity: i.quantity,
                    maximum_endurance: i.maximum_endurance, 
                    current_endurance: i.current_endurance, 
                    unknown1: 0, 
                    physical_attack_talisman: i.physical_attack_talisman, 
                    magical_attack_talisman: i.magical_attack_talisman, 
                    unknown2: vec![0], 
                    talisman_of_accuracy: i.talisman_of_accuracy, 
                    unknown3: vec![0], 
                    talisman_of_defence: i.talisman_of_defence, 
                    unknown4: vec![57], 
                    upgrade_level: i.upgrade_level, 
                    upgrade_rate: i.upgrade_rate, 
                    seconds_remaining: 0, 
                    unknown5: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }}).collect();
            let inventory = crate::packets::server::inventory::Inventory { items };
            socket_writer.write(&mut (&inventory).into());
        }
    }
}

#[derive(Component)]
pub struct Weapon {
    pub item: Option<Entity>
}

#[derive(Component)]
pub struct OldWeapon {
    pub item: Option<Entity>
}

fn equip_item(mut commands: Commands, mut query: Query<(Entity, &EquipItem, &mut Weapon)>, items_query: Query<(Entity, &PlayerOwner)>) {
    // println!("equip_itemSYS");
    for (entity, equip_item, mut weapon) in query.iter_mut() {
        for (weapon_entity, player_owner) in &items_query {
            // println!("ITEM {} {:?}", weapon_entity.index(), item);
            if weapon_entity.index() == equip_item.item_id {
                // println!("THIS IS THE ONE {} {:?}", weapon_entity.index(), item);
                if weapon.item.is_some() {
                    println!("SHOULD UNEQUIP");
                    weapon.item = None;
                } else {
                    // println!("WEAPON Is NonE");
                    if player_owner.player == entity {
                        weapon.item = Some(weapon_entity);
                        // println!("EQUIP {:?}", item);
                    }
                }
            }
        }
        // println!("TOGGLE: {:?} {:?}", entity, equip_item);
        // check if item is real
        // check if item is equiped and which part it is
        // if equiped -> remove item -> recalculate final points
        // if !equiped -> add item -> recalculate points
        // schedule save
        // fix appearence
        commands.entity(entity).remove::<EquipItem>();
    }
}

fn unequip_item(mut commands: Commands, mut query: Query<(Entity, &UnequipItem, &mut Weapon)>, items_query: Query<Entity>) {
    for (entity, unequip_item, mut weapon) in query.iter_mut() {
        println!("UNEQUIP {:?}", unequip_item);
        for weapon_entity in &items_query {
            if weapon_entity.index() == unequip_item.item_id {
                // if weapon.item.is_some() {
                    println!("SHOULD UNEQUIP");
                    weapon.item = None;
                // }
            }
        }
        commands.entity(entity).remove::<UnequipItem>();
    }
}

fn use_item(mut commands: Commands, query: Query<(Entity, &UseItem)>) {
    for (entity, use_item) in &query {
        println!("USE ITEM {:?}", use_item);
        commands.entity(entity).remove::<UseItem>();
    }
}

fn broadcast_weapon_change(mut query: Query<(Changed<Weapon>, &Player, &Weapon, &mut OldWeapon, &Position)>, players_query: Query<(&Position, &SocketWriter)>) {
    for (changed, player, weapon, mut old_weapon, position) in query.iter_mut() {
        if changed {
            if let Some(item) = weapon.item {
                let equip_item = crate::packets::server::equip_item::EquipItem { 
                    player_id: player.id, 
                    item_id: item.index(), 
                    item_index: 1 
                };
                for (other_position, other_socket_writer) in &players_query {
                    if other_position.is_in_sight(&position) {
                        other_socket_writer.write(&mut (&equip_item).into());
                    }
                }
            } else {
                if let Some(item) = old_weapon.item {
                    let unequip_item = crate::packets::server::unequip_item::UnequipItem { 
                        player_id: player.id, 
                        item_id: item.index(), 
                        item_index: 1 
                    };
                    for (other_position, other_socket_writer) in &players_query {
                        if other_position.is_in_sight(&position) {
                            other_socket_writer.write(&mut (&unequip_item).into());
                        }
                    }
                }
            }
            old_weapon.item = weapon.item;
        }
    }
}