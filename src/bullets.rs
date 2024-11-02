use bevy::prelude::*;

use crate::{
    assets::SceneAssets,
    collision::Collider,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    ship::Ship,
};

const BULLET_SPEED: f32 = 30.0;
const BULLET_ACCELERATION: f32 = 100.0;

pub struct BulletsPlugin;

impl Plugin for BulletsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BulletTimer(Timer::from_seconds(0.2, TimerMode::Repeating)));
        app.add_systems(Update, spawn_bullets);
        app.add_systems(FixedUpdate, check_bullets_out_of_reach);
    }
}

#[derive(Component)]
pub struct Bullet;

#[derive(Resource, Deref, DerefMut)]
struct BulletTimer(Timer);

fn spawn_bullets(
    mut commands: Commands,
    transform: Query<&Transform, With<Ship>>,
    assets: Res<SceneAssets>,
    mut bullet_timer: ResMut<BulletTimer>,
    time: Res<Time>,
) {
    bullet_timer.tick(time.delta());
    if !bullet_timer.just_finished() {
        return;
    }

    let transform = transform.single();
    commands.spawn((
        MovingObjectBundle {
            scene: SceneBundle {
                scene: assets.bullet.clone(),
                transform: Transform::from_translation(transform.translation + Vec3::Z * 2.5)
                    .with_scale(Vec3::splat(0.3)),
                ..default()
            },
            velocity: Velocity(Vec3::Z * BULLET_SPEED),
            acceleration: Acceleration(Vec3::Z * BULLET_ACCELERATION),
        },
        Bullet,
        Collider::new(1.0),
    ));
}

fn check_bullets_out_of_reach(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.z > 100.0 {
            commands.entity(entity).despawn();
        }
    }
}
