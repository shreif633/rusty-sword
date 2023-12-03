use bevy::prelude::*;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::medicine::Medicine;
use crate::components::cooldown::Cooldown;
use crate::components::visual_effect::VisualEffect;

pub struct MedicinePlugin;

impl Plugin for MedicinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, check_cooldown_for_medicine);
        app.add_systems(Update, apply_medicine);
        app.add_systems(Update, tick_medicine_cooldown);
    }
}

fn check_cooldown_for_medicine(mut commands: Commands, query: Query<(Entity, &Medicine), With<Cooldown<Medicine>>>) {
    for (entity, _medicine) in &query {
        commands.entity(entity).remove::<Medicine>();
    }
}

fn apply_medicine(mut commands: Commands, mut query: Query<(Entity, &mut CurrentHealthPoints, &Medicine), Without<Cooldown<Medicine>>>) {
    for (entity, mut current_health_points, medicine) in query.iter_mut() {
        current_health_points.current_health_points += medicine.health_recovered;
        commands.entity(entity).insert(VisualEffect { visual_effect: "effect_skill01".to_string() });
        commands.entity(entity).insert(Cooldown::<Medicine>::new(medicine.cooldown_in_seconds));
        commands.entity(entity).remove::<Medicine>();
    }
}

fn tick_medicine_cooldown(mut commands: Commands, mut query: Query<(Entity, &mut Cooldown<Medicine>)>, time: Res<Time>) {
    for (entity, mut cooldown) in query.iter_mut() {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.just_finished() {
            commands.entity(entity).remove::<Cooldown<Medicine>>();
        }
    }
}