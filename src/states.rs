use bevy::prelude::*;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum AppState {
    Title,
    Game,
    GameOver,
}

#[derive(Component)]
pub struct TransitionInto {
    state: AppState,
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
