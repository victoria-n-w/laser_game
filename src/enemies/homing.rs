use bevy::prelude::*;

use crate::{math::FrameChanger, movement::dynamics::Dynamics, player::entity::Player};

#[derive(Component)]
pub struct Movement;

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn move_homing_enemies(
    time: Res<Time>,
    mut player_query: Query<(&Player, &Transform), Without<Movement>>,
    mut enemies_query: Query<(&Movement, &Dynamics, &mut Transform), Without<Player>>,
) {
    let (_, player_in_world) = player_query
        .get_single_mut()
        .expect("Error: Could not find a single player.");

    enemies_query.for_each_mut(|(_, dynamics, mut enemy_in_world)| {
        let player_in_enemy_frame = enemy_in_world.local(*player_in_world).translation;
        let mut target_angle = Vec3::X.angle_between(player_in_enemy_frame);

        if target_angle.is_nan() {
            // this might occur only if the enemy is located directly on the player
            return;
        }

        if player_in_enemy_frame.y < 0.0 {
            target_angle *= -1.0;
        }

        assert!(-std::f32::consts::PI <= target_angle);
        assert!(target_angle <= std::f32::consts::PI);

        target_angle = target_angle.clamp(-dynamics.turning_speed, dynamics.turning_speed);

        enemy_in_world.rotate_z(target_angle);

        let delta_position = enemy_in_world.local_x() * dynamics.max_speed * time.delta_seconds();

        enemy_in_world.translation += delta_position;
    });
}
