use bevy::prelude::*;

use crate::components::controls::Controls;

use super::player::Player;

pub fn controls_system(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Controls, &mut Transform)>,
) {
}
