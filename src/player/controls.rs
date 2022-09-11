use bevy::prelude::*;

use crate::movement::controls::{Controls, Drive, Turn};

use super::{attack, entity};

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn process_keyboard_input(
    keyboard: Res<Input<KeyCode>>,
    mut attack_events: EventWriter<attack::Started>,
    mut player_query: Query<(&mut Controls, &mut attack::Attacking), With<entity::Player>>,
) {
    let (mut controls, mut attacking) = player_query.single_mut();

    match (keyboard.pressed(KeyCode::W), keyboard.pressed(KeyCode::S)) {
        (true, false) => controls.drive = Drive::Forwards,
        (false, true) => controls.drive = Drive::Backward,
        _ => controls.drive = Drive::Idle,
    }

    // repeating the values allows the match to be written in a clean and readeable way
    #[allow(clippy::match_same_arms)]
    match (
        keyboard.pressed(KeyCode::A),
        keyboard.pressed(KeyCode::D),
        &controls.drive,
    ) {
        (true, false, Drive::Backward) => controls.turn = Turn::Clockwise,
        (false, true, Drive::Backward) => controls.turn = Turn::Anticlockwise,
        (true, false, _) => controls.turn = Turn::Anticlockwise,
        (false, true, _) => controls.turn = Turn::Clockwise,
        _ => controls.turn = Turn::Idle,
    }

    if keyboard.pressed(KeyCode::Return) {
        if attacking.trigger() {
            attack_events.send_default();
        };
    };
}
