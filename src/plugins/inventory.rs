use bevy::prelude::*;
use crate::components::equipped_weapon::EquippedWeapon;
use crate::components::item::Item;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::repositories::item::find_all_items_by_player_id;
use crate::responses::equip_item::EquipItemResponse;
use crate::responses::inventory::InventoryResponse;
use crate::{framework::database::Database, responses::unequip_item::UnequipItemResponse};
use crate::requests::use_item::UseItemRequest;
use crate::requests::unequip_item::UnequipItemRequest;
use crate::requests::equip_item::EquipItemRequest;
use super::tcp_server::SocketWriter;

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
struct PlayerOwner {
    player: Entity
}

fn load_inventory(mut commands: Commands, query: Query<(Entity, Added<Player>, &Player, &SocketWriter)>, database: Res<Database>) {
    for (entity, added_player, player, socket_writer) in &query {
        if added_player {
            let items = find_all_items_by_player_id(&database, player.id);
            let items: Vec<(u32, Item)> = items.iter().map(|item_row| {
                let item = Item::from(item_row);
                let item_id = commands.spawn((item.clone(), PlayerOwner { player: entity })).id().index();
                (item_id, item)
            }).collect();
            let inventory = InventoryResponse::new(items);
            socket_writer.write(&mut (&inventory).into());
        }
    }
}

fn equip_item(mut commands: Commands, mut query: Query<(Entity, &EquipItemRequest, &mut EquippedWeapon)>, items_query: Query<(Entity, &PlayerOwner)>) {
    for (entity, equip_item, mut weapon) in query.iter_mut() {
        for (weapon_entity, player_owner) in &items_query {
            if weapon_entity.index() == equip_item.item_id {
                if weapon.item.is_some() {
                    weapon.item = None;
                } else {
                    if player_owner.player == entity {
                        weapon.item = Some(weapon_entity);
                    }
                }
            }
        }
        commands.entity(entity).remove::<EquipItemRequest>();
    }
}

fn unequip_item(mut commands: Commands, mut query: Query<(Entity, &UnequipItemRequest, &mut EquippedWeapon)>, items_query: Query<Entity>) {
    for (entity, unequip_item, mut weapon) in query.iter_mut() {
        for weapon_entity in &items_query {
            if weapon_entity.index() == unequip_item.item_id {
                weapon.item = None;
            }
        }
        commands.entity(entity).remove::<UnequipItemRequest>();
    }
}

fn use_item(mut commands: Commands, query: Query<(Entity, &UseItemRequest)>) {
    for (entity, use_item) in &query {
        println!("USE ITEM {:?}", use_item);
        commands.entity(entity).remove::<UseItemRequest>();
    }
}

fn broadcast_weapon_change(mut query: Query<(Changed<EquippedWeapon>, &Player, &EquippedWeapon, &mut Previous<EquippedWeapon>, &Position)>, players_query: Query<(&Position, &SocketWriter)>) {
    for (changed, player, weapon, mut old_weapon, position) in query.iter_mut() {
        if changed {
            if let Some(item) = weapon.item {
                let equip_item = EquipItemResponse { 
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
                if let Some(item) = old_weapon.entity.item {
                    let unequip_item = UnequipItemResponse { 
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
            old_weapon.entity.item = weapon.item;
        }
    }
}