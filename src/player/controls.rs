use bevy::prelude::*;

use crate::movement::controls::{Controls, Drive, Turn};

use super::entity::Player;

pub fn player_controls(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Controls)>,
) {
    let (_, mut controls) = player_query.single_mut();

    match (keyboard.pressed(KeyCode::W), keyboard.pressed(KeyCode::S)) {
        (true, false) => controls.drive = Drive::Forwards,
        (false, true) => controls.drive = Drive::Backward,
        _ => controls.drive = Drive::Idle,
    }

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
}
