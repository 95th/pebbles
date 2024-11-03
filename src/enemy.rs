use std::{f32::consts::PI, ops::Range};

use bevy::{math::*, prelude::*};
use rand::Rng;

use crate::{
    assets::SceneAssets,
    collision::{Collider, Damage},
    despawn::DespawnWhenFar,
    health::Health,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    schedule::GameSchedule,
};

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const VELOCITY_SCALE: f32 = 10.0;
const SPAWN_TIME_SECONDS: f32 = 3.0;

const ENEMY_HEALTH: f32 = 100.0;
const ENEMY_DAMAGE: f32 = 10.0; // Exactly the same as the bullet health

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(
            SPAWN_TIME_SECONDS,
            TimerMode::Repeating,
        )));

        app.add_systems(Update, spawn_enemies.in_set(GameSchedule::EntityUpdates));
    }
}

#[derive(Resource, Deref, DerefMut)]
struct SpawnTimer(Timer);

#[derive(Component)]
pub struct Enemy;

fn spawn_enemies(
    mut spawn_timer: ResMut<SpawnTimer>,
    commands: Commands,
    time: Res<Time>,
    assets: Res<SceneAssets>,
) {
    spawn_timer.tick(time.delta());
    if !spawn_timer.just_finished() {
        return;
    }

    spawn_enemy_wave(commands, assets, 5);
}

fn spawn_enemy_wave(mut commands: Commands, assets: Res<SceneAssets>, count: u8) {
    let mut translation = vec3(rand::thread_rng().gen_range(SPAWN_RANGE_X), 0.0, 30.0);
    let velocity = Vec3::Z * -VELOCITY_SCALE;

    for _ in 0..count {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity(velocity),
                acceleration: Acceleration(Vec3::ZERO),
                scene: SceneBundle {
                    scene: assets.enemy.clone(),
                    transform: Transform::from_translation(translation)
                        .with_rotation(Quat::from_rotation_y(PI)),
                    ..default()
                },
                collider: Collider::new(2.0),
            },
            DespawnWhenFar,
            Enemy,
            Health::new(ENEMY_HEALTH),
            Damage::new(ENEMY_DAMAGE),
        ));

        translation.z += 5.0;
    }
}
