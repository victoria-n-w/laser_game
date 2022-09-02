use bevy::prelude::*;

use crate::{movement::controls::Controls, player::entity::Player};

#[derive(Component)]
pub struct HomingMovement;

pub fn control_homing_enemies(
    mut player_query: Query<(&Player, &Transform)>,
    mut enemies_query: Query<(&HomingMovement, &mut Controls, &Transform)>,
) {
    let (_, mut player_position) = player_query
        .get_single_mut()
        .expect("Error: Could not find a single player.");

    enemies_query.for_each(|(_, mut controls, enemy_position)| -> () {})
}
