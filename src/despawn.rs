use bevy::prelude::*;

use crate::movement::MovingObject;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_objects);
    }
}

const DESPAWN_DISTANCE: f32 = 100.0;

fn despawn_far_objects(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), With<MovingObject>>,
) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}
