use bevy::prelude::*;

mod assets;
mod bullets;
mod camera;
mod collision;
mod despawn;
mod enemy;
mod game;
mod movement;
mod schedule;
mod ship;
mod state;
mod window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game::GamePlugin)
        .run();
}
