use std::collections::HashMap;

use bevy::prelude::*;

use crate::schedule::GameSchedule;

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_collision.in_set(GameSchedule::CollisionDetection),
        );
    }
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Collider {
            radius,
            colliding_entities: Vec::new(),
        }
    }
}

fn check_collision(mut query: Query<(Entity, &Transform, &mut Collider)>) {
    let mut colliding_entity_map = HashMap::<Entity, Vec<Entity>>::new();

    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a == entity_b {
                continue;
            }

            let distance = transform_a.translation.distance(transform_b.translation);

            if distance < collider_a.radius + collider_b.radius {
                colliding_entity_map
                    .entry(entity_a)
                    .or_insert_with(Vec::new)
                    .push(entity_b);
            }
        }
    }

    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_entities.clear();
        if let Some(colliding_entities) = colliding_entity_map.get(&entity) {
            collider.colliding_entities.extend(colliding_entities);
        }
    }
}
