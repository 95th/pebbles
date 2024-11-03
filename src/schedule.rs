use bevy::prelude::*;

use crate::state::GameState;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, SystemSet)]
pub enum GameSchedule {
    DespawnEntities,
    UserInput,
    EntityUpdates,
    CollisionDetection,
}

pub struct GameSchedulePlugin;

impl Plugin for GameSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSchedule::DespawnEntities,
                GameSchedule::UserInput,
                GameSchedule::EntityUpdates,
                GameSchedule::CollisionDetection,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );

        app.add_systems(
            Update,
            apply_deferred
                .after(GameSchedule::DespawnEntities)
                .before(GameSchedule::UserInput),
        );
    }
}
