use bevy::{prelude::*, window::CursorGrabMode};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_window);
    }
}

fn setup_window(mut window: Query<&mut Window>) {
    let mut window = window.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Confined;
}
