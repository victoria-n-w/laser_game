use bevy::prelude::*;

use crate::states;

pub fn any_key_next_screen(
    keys: Res<Input<KeyCode>>,
    state: Res<State<states::AppState>>,
    mut publisher: EventWriter<states::TransitionInto>,
) {
    if keys.get_just_pressed().len() > 0 {
        publisher.send(states::TransitionInto {
            state: state.current().next(),
        });
    };
}
