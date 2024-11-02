use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::Y * CAMERA_DISTANCE)
            .looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });

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
        transform: Transform::from_translation(Vec3::new(-1000.0, 50.0, 1000.0))
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
