use bevy::prelude::*;
use crate::components::equipped_weapon::EquippedWeapon;
use crate::components::id::Id;
use crate::components::item::Item;
use crate::components::item_quantity::ItemQuantity;
use crate::components::medicine::Medicine;
use crate::components::network_writer::NetworkWriter;
use crate::components::player::Player;
use crate::components::player_owner::PlayerOwner;
use crate::components::position::Position;
use crate::components::previous::Previous;
use crate::configs::items::{ItemsConfig, ItemCategory};
use crate::framework::entity_map::EntityMap;
use crate::repositories::item::find_all_items_by_player_id;
use crate::responses::equip_item::EquipItemResponse;
use crate::responses::inventory::InventoryResponse;
use crate::responses::update_item_quantity::{UpdateItemQuantityResponse, ItemQuantityAction};
use crate::framework::database::Database;
use crate::responses::unequip_item::UnequipItemResponse;
use crate::requests::use_item::UseItemRequest;
use crate::requests::unequip_item::UnequipItemRequest;
use crate::requests::equip_item::EquipItemRequest;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, use_item);
        app.add_systems(Update, load_inventory);
        app.add_systems(Update, equip_item.before(broadcast_weapon_change));
        app.add_systems(Update, unequip_item.before(broadcast_weapon_change));
        app.add_systems(Update, broadcast_weapon_change);
        app.add_systems(PostUpdate, update_item_quantity);
    }
}

fn load_inventory(mut commands: Commands, query: Query<(Entity, &Id, &NetworkWriter), Added<Player>>, database: Res<Database>, mut items_map: ResMut<EntityMap<Item>>) {
    for (entity, id, socket_writer) in &query {
        let items = find_all_items_by_player_id(&database, id.id);
        let items: Vec<Item> = items.iter().map(|item_row| {
            let item = Item::from(item_row);
            let item_entity = commands.spawn((item.clone(), PlayerOwner { player: entity }, ItemQuantity { quantity: item_row.quantity })).id();
            items_map.map.insert(item_row.id, item_entity);
            item
        }).collect();
        let inventory = InventoryResponse::new(items);
        socket_writer.write(&mut (&inventory).into());
    }
}

fn equip_item(mut commands: Commands, mut query: Query<(Entity, &EquipItemRequest, &mut EquippedWeapon)>, items_query: Query<(&Item, &PlayerOwner)>, items_map: Res<EntityMap<Item>>) {
    for (entity, equip_item, mut weapon) in query.iter_mut() {
        if let Some(item_entity) = items_map.map.get(&equip_item.item_id) {
            if let Ok((item, player_owner)) = items_query.get(*item_entity) {
                if weapon.item_id.is_some() {
                    weapon.item_id = None;
                } else if player_owner.player == entity {
                    weapon.item_id = Some(item.id);
                }
            }
        }
        commands.entity(entity).remove::<EquipItemRequest>();
    }
}

fn unequip_item(mut commands: Commands, mut query: Query<(Entity, &UnequipItemRequest, &mut EquippedWeapon)>, items_query: Query<(&Item, &PlayerOwner)>, items_map: Res<EntityMap<Item>>) {
    for (entity, unequip_item, mut weapon) in query.iter_mut() {
        if let Some(equiped_item_id) = weapon.item_id {
            if let Some(item_entity) = items_map.map.get(&unequip_item.item_id) {
                if let Ok((item, player_owner)) = items_query.get(*item_entity) {
                    if player_owner.player == entity && item.id == equiped_item_id {
                        weapon.item_id = None;
                    }
                }
            }
        }
        commands.entity(entity).remove::<UnequipItemRequest>();
    }
}

fn use_item(mut commands: Commands, query: Query<(Entity, &UseItemRequest)>, mut items_query: Query<(&Item, &mut ItemQuantity)>, items_config: Res<ItemsConfig>, items_map: Res<EntityMap<Item>>) {
    for (entity, use_item) in &query {
        if let Some(item_entity) = items_map.map.get(&use_item.item_id) {
            if let Ok((item, mut item_quantity)) = items_query.get_mut(*item_entity) {
                let config = items_config.config.get(&item.index);
                if let Some(config) = config {
                    if config.category == ItemCategory::Medicine {
                        commands.entity(entity).insert(Medicine { 
                            health_recovered: config.health_recovered, 
                            cooldown_in_seconds: config.cooldown_in_seconds 
                        });
                    }
                    if config.consumable {
                        item_quantity.quantity -= 1
                    }
                }
            }
        }
        commands.entity(entity).remove::<UseItemRequest>();
    }
}

fn update_item_quantity(query: Query<(&Item, &ItemQuantity, &PlayerOwner), Changed<ItemQuantity>>, players_query: Query<&NetworkWriter>) {
    for (item, item_quantity, player_owner) in &query {
        let update_item_quantity_response = UpdateItemQuantityResponse { item_id: item.id, quantity: item_quantity.quantity, action: ItemQuantityAction::Consume };
        if let Ok(socket_writer) = players_query.get(player_owner.player) {
            socket_writer.write(&mut (&update_item_quantity_response).into());
        }
    }
}

fn broadcast_weapon_change(mut query: Query<(&Id, &EquippedWeapon, &mut Previous<EquippedWeapon>, &Position), Changed<EquippedWeapon>>, players_query: Query<(&Position, &NetworkWriter)>) {
    for (id, weapon, mut old_weapon, position) in query.iter_mut() {
        if let Some(item_id) = weapon.item_id {
            let equip_item = EquipItemResponse { 
                player_id: id.id, 
                item_id, 
                item_index: 1 
            };
            for (other_position, other_socket_writer) in &players_query {
                if other_position.is_in_sight(position) {
                    other_socket_writer.write(&mut (&equip_item).into());
                }
            }
        } else if let Some(item_id) = old_weapon.entity.item_id {
            let unequip_item = UnequipItemResponse { 
                player_id: id.id, 
                item_id, 
                item_index: 1 
            };
            for (other_position, other_socket_writer) in &players_query {
                if other_position.is_in_sight(position) {
                    other_socket_writer.write(&mut (&unequip_item).into());
                }
            }
        }
        old_weapon.entity.item_id = weapon.item_id;
    }
}