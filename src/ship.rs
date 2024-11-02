use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::assets::SceneAssets;

const BOUNDS_LEFT: f32 = -40.0;
const BOUNDS_RIGHT: f32 = 40.0;
const BOUNDS_TOP: f32 = 20.0;
const BOUNDS_BOTTOM: f32 = -20.0;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ship);
        app.add_systems(Update, ship_movement);
    }
}

#[derive(Component)]
pub struct Ship;

fn spawn_ship(mut commands: Commands, assets: Res<SceneAssets>) {
    commands.spawn((
        SceneBundle {
            scene: assets.ship.clone(),
            ..default()
        },
        Ship,
    ));
}

fn ship_movement(
    mut mouse_motion: EventReader<MouseMotion>,
    mut transform: Query<&mut Transform, With<Ship>>,
) {
    let mut transform = transform.single_mut();
    for motion in mouse_motion.read() {
        transform.translation.x -= motion.delta.x * 0.1;
        transform.translation.z -= motion.delta.y * 0.1;
    }

    transform.translation.x = transform.translation.x.clamp(BOUNDS_LEFT, BOUNDS_RIGHT);
    transform.translation.z = transform.translation.z.clamp(BOUNDS_BOTTOM, BOUNDS_TOP);
}
