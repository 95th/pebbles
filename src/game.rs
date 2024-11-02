use bevy::prelude::*;

use crate::assets::SceneAssetLoaderPlugin;
use crate::bullets::BulletsPlugin;
use crate::camera::CameraPlugin;
use crate::collision::CollisionDetectionPlugin;
use crate::enemy::EnemyPlugin;
use crate::movement::MovementPlugin;
use crate::ship::ShipPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)));

        app.add_plugins(SceneAssetLoaderPlugin);
        app.add_plugins(CameraPlugin);
        app.add_plugins(ShipPlugin);
        app.add_plugins(EnemyPlugin);
        app.add_plugins(BulletsPlugin);
        app.add_plugins(MovementPlugin);
        app.add_plugins(CollisionDetectionPlugin);
    }
}
