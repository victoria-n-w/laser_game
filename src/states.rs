use bevy::prelude::*;

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

fn system(mut event: EventReader<TransitionInto>, mut state: ResMut<State<AppState>>) {
    for into in event.iter() {
        let _ = state.replace(into.state.clone());
    }
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TransitionInto>().add_system(system);
    }
}
