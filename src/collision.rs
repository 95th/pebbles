use bevy::prelude::*;

use crate::{bullets::Bullet, enemy::Enemy, health::Health, schedule::GameSchedule};

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
struct CollisionEvent(Entity, Entity);

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
    bullets: Query<(Entity, &Transform, &Collider), With<Bullet>>,
    enemies: Query<(Entity, &Transform, &Collider), With<Enemy>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
    for (bullet_entity, bullet_transform, bullet_collider) in bullets.iter() {
        for (enemy_entity, enemy_transform, enemy_collider) in enemies.iter() {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);

            if distance < bullet_collider.radius + enemy_collider.radius {
                collision_event_writer.send_batch([
                    CollisionEvent(bullet_entity, enemy_entity),
                    CollisionEvent(enemy_entity, bullet_entity),
                ]);
            }
        }
    }
}

fn apply_collision_damage(
    mut collision_event_reader: EventReader<CollisionEvent>,
    mut healths: Query<&mut Health>,
    damages: Query<&Damage>,
) {
    for &CollisionEvent(a, b) in collision_event_reader.read() {
        let Ok(mut health) = healths.get_mut(a) else {
            continue;
        };
        let Ok(damage) = damages.get(b) else {
            continue;
        };
        health.value -= damage.amount;
    }
}
