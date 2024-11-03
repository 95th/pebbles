use std::{f32::consts::PI, ops::Range};

use bevy::{math::*, prelude::*};
use rand::Rng;

use crate::{
    assets::SceneAssets,
    bullets::Bullet,
    collision::Collider,
    despawn::DespawnWhenFar,
    movement::{Acceleration, MovingObjectBundle, Velocity},
    state::GameState,
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

        app.add_systems(
            Update,
            (spawn_enemy, check_bullet_collision).run_if(in_state(GameState::Playing)),
        );
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
            collider: Collider::new(2.0),
        },
        DespawnWhenFar,
        Enemy,
    ));
}

fn check_bullet_collision(
    mut commands: Commands,
    enemies: Query<(Entity, &Collider), With<Enemy>>,
    bullets: Query<(), With<Bullet>>,
) {
    for (enemy_entity, enemy_collider) in enemies.iter() {
        for &colliding_entity in enemy_collider.colliding_entities.iter() {
            if bullets.get(colliding_entity).is_ok() {
                commands.entity(colliding_entity).despawn_recursive();
                commands.entity(enemy_entity).despawn_recursive();
            }
        }
    }
}
