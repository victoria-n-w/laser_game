use bevy::prelude::*;

use crate::{
    math::relativity::FrameChanger,
    movement::simple_moves::SimpleControls,
    player::{self, entity::Player},
};

use super::entity;

#[derive(Component, Default)]
pub struct Navigation;

impl entity::Navigation for Navigation {
    fn texture_path() -> String {
        String::from("blue.png")
    }
}

pub fn navigation_system(
    mut player_query: Query<&Transform, With<player::entity::Player>>,
    mut enemies_query: Query<
        (&mut SimpleControls, &Transform),
        (Without<Player>, With<Navigation>),
    >,
) {
    let player_in_world = player_query
        .get_single_mut()
        .expect("Error: Could not find a single player.");

    enemies_query.for_each_mut(|(mut controls, enemy_in_world)| {
        let player_in_enemy_frame = enemy_in_world.local(*player_in_world).translation;
        let mut target_angle = Vec3::X.angle_between(player_in_enemy_frame);

        if player_in_enemy_frame.y < 0_f32 {
            target_angle *= -1_f32;
        }

        controls.set_target_angle(target_angle);
    });
}
