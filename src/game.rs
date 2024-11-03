use bevy::prelude::*;

use crate::assets::SceneAssetLoaderPlugin;
use crate::bullets::BulletsPlugin;
use crate::camera::CameraPlugin;
use crate::collision::CollisionDetectionPlugin;
use crate::despawn::DespawnPlugin;
use crate::enemy::EnemyPlugin;
use crate::movement::MovementPlugin;
use crate::schedule::GameSchedulePlugin;
use crate::ship::ShipPlugin;
use crate::state::GameStatePlugin;
use crate::window::WindowPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)));

        app.add_plugins(WindowPlugin);
        app.add_plugins(GameStatePlugin);
        app.add_plugins(GameSchedulePlugin);
        app.add_plugins(SceneAssetLoaderPlugin);
        app.add_plugins(CameraPlugin);
        app.add_plugins(ShipPlugin);
        app.add_plugins(EnemyPlugin);
        app.add_plugins(BulletsPlugin);
        app.add_plugins(MovementPlugin);
        app.add_plugins(CollisionDetectionPlugin);
        app.add_plugins(DespawnPlugin);
    }
}
