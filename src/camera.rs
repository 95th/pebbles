use bevy::{core_pipeline::bloom::BloomSettings, prelude::*};

use crate::{schedule::GameSchedule, ship::Ship, state::GameState};

const CAMERA_DISTANCE: f32 = 80.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, follow_ship.in_set(GameSchedule::UserInput));
        app.add_systems(OnEnter(GameState::Paused), blur_camera);
        app.add_systems(OnExit(GameState::Paused), unblur_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_translation(Vec3::Y * CAMERA_DISTANCE)
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        BloomSettings::NATURAL,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 800.0,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 50.0, -20.0))
            .looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::new(500.0, 2.0, 500.0)),
        material: materials.add(StandardMaterial::from_color(Color::srgb(0.1, 0.1, 0.14))),
        transform: Transform::from_translation(Vec3::new(0.0, -4.0, 0.0)),
        ..default()
    });
}

fn follow_ship(
    mut query: Query<&mut Transform, With<Camera>>,
    ship_query: Query<&Transform, (With<Ship>, Without<Camera>)>,
) {
    let ship_transform = ship_query.single();
    let mut camera_transform = query.single_mut();
    camera_transform.translation.x = ship_transform.translation.x / 8.0;
    camera_transform.translation.z = ship_transform.translation.z / 8.0;
}

fn blur_camera(mut bloom_settings: Query<&mut BloomSettings, With<Camera>>) {
    let mut bloom_settings = bloom_settings.single_mut();
    *bloom_settings = BloomSettings::SCREEN_BLUR;
}

fn unblur_camera(mut bloom_settings: Query<&mut BloomSettings, With<Camera>>) {
    let mut bloom_settings = bloom_settings.single_mut();
    *bloom_settings = BloomSettings::NATURAL;
}
