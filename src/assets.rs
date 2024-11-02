use bevy::prelude::*;

pub struct SceneAssetLoaderPlugin;

impl Plugin for SceneAssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SceneAssets::default());
        app.add_systems(Startup, load_assets);
    }
}

#[derive(Resource, Default)]
pub struct SceneAssets {
    pub enemy: Handle<Scene>,
    pub ship: Handle<Scene>,
    pub bullet: Handle<Scene>,
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, assets: Res<AssetServer>) {
    scene_assets.enemy = assets.load("enemy.glb#Scene0");
    scene_assets.ship = assets.load("ship.glb#Scene0");
    scene_assets.bullet = assets.load("bullet.glb#Scene0");
}
