
use bevy::prelude::*;
use crate::components::item::Item;
use crate::components::item_quantity::ItemQuantity;
use crate::repositories::item::{ItemUpdateQuantityChangeset, update_all_item_quantity_by_id};
use crate::framework::database::Database;

pub struct PersistItemPlugin;

impl Plugin for PersistItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_persist_item_timer);
        app.add_systems(Last, schedule_for_persistence);
        app.add_systems(Last, persist_items_quantities);
    }
}

#[derive(Component)]
struct PersistItemTimer {
    timer: Timer,
}

fn create_persist_item_timer(mut commands: Commands) {
    commands.spawn(PersistItemTimer { timer: Timer::from_seconds(30.0, TimerMode::Repeating) });
}

#[derive(Component)]
struct ShouldPersist;

fn schedule_for_persistence(mut commands: Commands, query: Query<Entity, Changed<ItemQuantity>>) {
    for entity in &query {
        commands.entity(entity).insert(ShouldPersist);
    }
}   

fn persist_items_quantities(
    mut commands: Commands, 
    mut timer: Query<&mut PersistItemTimer>, 
    query: Query<(Entity, &Item, &ItemQuantity), With<ShouldPersist>>, 
    database: Res<Database>, 
    time: Res<Time>
) {
    if let Ok(mut timer) = timer.get_single_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            let mut changesets = Vec::<ItemUpdateQuantityChangeset>::new();
            for (entity, item, item_quantity) in &query {
                let changeset = ItemUpdateQuantityChangeset { id: item.id, quantity: item_quantity.quantity };
                changesets.push(changeset);
                commands.entity(entity).remove::<ShouldPersist>();
            }
            update_all_item_quantity_by_id(&database, &changesets);
        }
    }
}   