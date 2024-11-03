use bevy::prelude::*;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_systems(Update, pause_toggle_system);
    }
}

fn pause_toggle_system(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    kbd: ResMut<ButtonInput<KeyCode>>,
) {
    if !kbd.just_pressed(KeyCode::Escape) {
        return;
    }

    match state.get() {
        GameState::Playing => {
            next_state.set(GameState::Paused);
        }
        GameState::Paused => {
            next_state.set(GameState::Playing);
        }
    }
}
