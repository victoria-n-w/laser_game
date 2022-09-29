use bevy::prelude::*;

use crate::states;

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn any_key_next_screen(
    mut keys: ResMut<Input<KeyCode>>,
    state: Res<State<states::AppState>>,
    mut publisher: EventWriter<states::TransitionInto>,
) {
    if keys.get_just_pressed().len() > 0 {
        publisher.send(states::TransitionInto {
            state: state.current().next(),
        });
        keys.reset_all();
    };
}
