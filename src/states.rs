use bevy::prelude::*;

use crate::util;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum AppState {
    Title,
    Game,
    GameOver,
}

impl AppState {
    pub fn next(&self) -> Self {
        match self {
            Self::Title => Self::Game,
            Self::Game => Self::GameOver,
            Self::GameOver => Self::Title,
        }
    }
}

pub struct TransitionInto {
    pub state: AppState,
}

/// automatically despawns all entities with Persistent component
pub fn transitioning(
    mut event: EventReader<TransitionInto>,
    mut state: ResMut<State<AppState>>,
    mut commands: Commands,
) {
    for into in event.iter() {
        match state.replace(into.state.clone()) {
            Ok(_) => commands.add(util::commands::DespawnAll),
            Err(_) => (),
        };
    }
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TransitionInto>().add_system(transitioning);
    }
}
