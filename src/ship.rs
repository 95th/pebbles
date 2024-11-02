use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

use crate::assets::SceneAssets;

const BOUNDS_LEFT: f32 = -40.0;
const BOUNDS_RIGHT: f32 = 40.0;
const BOUNDS_TOP: f32 = 20.0;
const BOUNDS_BOTTOM: f32 = -20.0;

pub struct ShipPlugin;

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ship);
        app.add_systems(Update, (ship_movement, check_bounds));
    }
}

#[derive(Component)]
pub struct Ship;

fn spawn_ship(mut commands: Commands, assets: Res<SceneAssets>, mut window: Query<&mut Window>) {
    commands.spawn((
        SceneBundle {
            scene: assets.ship.clone(),
            ..default()
        },
        Ship,
    ));
    let mut window = window.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Confined;
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
}

fn check_bounds(mut transform: Query<&mut Transform, With<Ship>>) {
    let mut transform = transform.single_mut();
    if transform.translation.x < BOUNDS_LEFT {
        transform.translation.x = BOUNDS_LEFT;
    } else if transform.translation.x > BOUNDS_RIGHT {
        transform.translation.x = BOUNDS_RIGHT;
    }

    if transform.translation.z < BOUNDS_BOTTOM {
        transform.translation.z = BOUNDS_BOTTOM;
    } else if transform.translation.z > BOUNDS_TOP {
        transform.translation.z = BOUNDS_TOP;
    }
}
