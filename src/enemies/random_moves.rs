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
pub struct RandomMoves {
    pub timer: Timer,
}

pub fn control_random_moves(
    time: Res<Time>,
    mut objects_query: Query<(&mut RandomMoves, &mut Controls)>,
) {
    objects_query.for_each_mut(|(mut moves, mut controls)| -> () {
        moves.timer.tick(time.delta());
        if moves.timer.finished() {
            controls.turn = rand::random();
            moves.timer = Timer::new(
                Duration::from_secs_f32(rand::thread_rng().gen_range(0.0..5.0)),
                false,
            )
        }
    })
}
