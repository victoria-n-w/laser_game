use std::time::Duration;

use bevy::prelude::*;

use crate::movement::simple_moves::SimpleControls;

use rand::Rng;

#[derive(Component, Default)]
pub struct Navigation {
    pub timer: Timer,
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn generate_random_controls(
    time: Res<Time>,
    mut entities_query: Query<(&mut Navigation, &mut SimpleControls)>,
) {
    entities_query.for_each_mut(|(mut moves, mut controls)| {
        moves.timer.tick(time.delta());
        if moves.timer.finished() {
            let target = rand::thread_rng()
                .gen_range(-controls.max_angular_velocity..controls.max_angular_velocity);
            controls.set_angular_velocity(target);

            moves.timer = Timer::new(
                Duration::from_secs_f32(rand::thread_rng().gen_range(0.0..2.0)),
                false,
            );
        }
    });
}
