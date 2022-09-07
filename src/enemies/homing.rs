use bevy::prelude::*;

use crate::{math::FrameChanger, movement::simple_moves::SimpleControls, player::entity::Player};

#[derive(Component, Default)]
pub struct Navigation;

pub fn move_homing_enemies(
    mut player_query: Query<(&Player, &Transform)>,
    mut enemies_query: Query<(&Navigation, &mut SimpleControls, &Transform), Without<Player>>,
) {
    let (_, player_in_world) = player_query
        .get_single_mut()
        .expect("Error: Could not find a single player.");

    enemies_query.for_each_mut(|(_, mut controls, enemy_in_world)| {
        let player_in_enemy_frame = enemy_in_world.local(*player_in_world).translation;
        let mut target_angle = Vec3::X.angle_between(player_in_enemy_frame);

        if player_in_enemy_frame.y < 0_f32 {
            target_angle *= -1_f32;
        }

        controls.set_target_angle(target_angle);
    });
}
