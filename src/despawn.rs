use bevy::prelude::*;

use crate::{health::Health, schedule::GameSchedule};

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_far_objects, despawn_dead_objects).in_set(GameSchedule::DespawnEntities),
        );
    }
}

#[derive(Component)]
pub struct DespawnWhenFar;

const DESPAWN_DISTANCE: f32 = 100.0;

fn despawn_far_objects(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), With<DespawnWhenFar>>,
) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_dead_objects(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
