use bevy::prelude::*;

mod assets;
mod bullets;
mod camera;
mod enemy;
mod game;
mod movement;
mod ship;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game::GamePlugin)
        .run();
}
