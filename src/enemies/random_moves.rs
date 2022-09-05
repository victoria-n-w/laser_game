use std::time::Duration;

use bevy::prelude::*;

use crate::movement::controls::{Controls, Turn};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<Turn> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Turn {
        match rng.gen_range(0..=2) {
            0 => Turn::Idle,
            1 => Turn::Anticlockwise,
            _ => Turn::Clockwise,
        }
    }
}

#[derive(Component)]
pub struct RandomControlls {
    pub timer: Timer,
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn generate_random_controls(
    time: Res<Time>,
    mut entities_query: Query<(&mut RandomControlls, &mut Controls)>,
) {
    entities_query.for_each_mut(|(mut moves, mut controls)| {
        moves.timer.tick(time.delta());
        if moves.timer.finished() {
            controls.turn = rand::random();
            moves.timer = Timer::new(
                Duration::from_secs_f32(rand::thread_rng().gen_range(0.0..2.0)),
                false,
            );
        }
    });
}
