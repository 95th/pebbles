use std::{f32::consts::PI, ops::Range};

use bevy::{math::*, prelude::*};
use rand::Rng;

use crate::{
    assets::SceneAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;

const VELOCITY_SCALE: f32 = 5.0;
const ACCELERATION_SCALE: f32 = 1.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(
            SPAWN_TIME_SECONDS,
            TimerMode::Repeating,
        )));

        app.add_systems(Update, spawn_enemy);
        app.add_systems(FixedUpdate, check_enemy_out_of_reach);
    }
}

#[derive(Resource, Deref, DerefMut)]
struct SpawnTimer(Timer);

#[derive(Component)]
pub struct Enemy;

fn spawn_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    assets: Res<SceneAssets>,
) {
    spawn_timer.tick(time.delta());
    if !spawn_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let translation = vec3(
        rng.gen_range(SPAWN_RANGE_X),
        0.0,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    let mut random_unit_vector =
        || vec3(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0)).normalize_or_zero();

    let velocity = random_unit_vector() * VELOCITY_SCALE;
    let acceleration = random_unit_vector() * ACCELERATION_SCALE;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity(velocity),
            acceleration: Acceleration(acceleration),
            scene: SceneBundle {
                scene: assets.enemy.clone(),
                transform: Transform::from_translation(translation)
                    .with_rotation(Quat::from_rotation_y(PI)),
                ..default()
            },
        },
        Enemy,
    ));
}

fn check_enemy_out_of_reach(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.z < -100.0
            || transform.translation.z > 100.0
            || transform.translation.x < -100.0
            || transform.translation.x > 100.0
        {
            commands.entity(entity).despawn();
        }
    }
}
