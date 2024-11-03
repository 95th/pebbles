use bevy::prelude::*;

use crate::{health::Health, schedule::GameSchedule};

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_systems(
            Update,
            check_collision.in_set(GameSchedule::CollisionDetection),
        );
        app.add_systems(
            Update,
            apply_collision_damage.in_set(GameSchedule::EntityUpdates),
        );
    }
}

#[derive(Component)]
pub struct Damage {
    pub amount: f32,
}

impl Damage {
    pub fn new(amount: f32) -> Self {
        Damage { amount }
    }
}

#[derive(Event)]
pub struct CollisionEvent {
    pub source: Entity,
    pub target: Entity,
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Collider { radius }
    }
}

fn check_collision(
    sources: Query<(Entity, &Transform, &Collider), With<Damage>>,
    targets: Query<(Entity, &Transform, &Collider), With<Health>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    for (src_entity, src_transform, src_collider) in sources.iter() {
        for (target_entity, target_transform, target_collider) in targets.iter() {
            if src_entity == target_entity {
                continue;
            }

            let distance = src_transform
                .translation
                .distance(target_transform.translation);

            if distance < src_collider.radius + target_collider.radius {
                collision_event_writer.send(CollisionEvent {
                    source: src_entity,
                    target: target_entity,
                });
            }
        }
    }
}

fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut health_query: Query<&mut Health>,
    collision_damage_query: Query<&Damage>,
) {
    for event in collision_event_reader.read() {
        let Ok(mut health) = health_query.get_mut(event.target) else {
            continue;
        };
        let Ok(collision_damage) = collision_damage_query.get(event.source) else {
            continue;
        };
        health.value -= collision_damage.amount;
    }
}
