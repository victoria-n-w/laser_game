use std::time::Duration;

use bevy::prelude::*;

enum State {
    OnCooldown(Timer),
    Active(Timer),
}

impl Default for State {
    fn default() -> Self {
        Self::OnCooldown(Timer::from_seconds(0_f32, false))
    }
}

#[derive(Component, Default)]
pub struct Attacking {
    cooldown_time: f32,
    duration_time: f32,
    state: State,
}

impl Attacking {
    pub fn new(cooldown: f32, duration: f32) -> Self {
        Attacking {
            cooldown_time: cooldown,
            duration_time: duration,
            ..default()
        }
    }

    /// trigerring the attack when it's active or when it's on cooldown will do nothing
    pub fn trigger(&mut self) {
        match &self.state {
            State::OnCooldown(timer) => {
                if timer.finished() {
                    self.state = State::Active(Timer::from_seconds(self.duration_time, false));
                    println!("ATTACKING!!!")
                };
            }
            State::Active(_) => (),
        };
    }

    pub fn pass_time(&mut self, dtime: Duration) {
        match &mut self.state {
            State::OnCooldown(timer) => {
                timer.tick(dtime);
            }
            State::Active(timer) => {
                timer.tick(dtime);
                if timer.finished() {
                    self.state = State::OnCooldown(Timer::from_seconds(self.cooldown_time, false));
                }
            }
        };
    }

    pub fn is_active(&self) -> bool {
        match self.state {
            State::OnCooldown(_) => false,
            State::Active(_) => true,
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn system(time: Res<Time>, mut entities: Query<&mut Attacking>) {
    entities.for_each_mut(|mut attacking| {
        attacking.pass_time(time.delta());
    });
}
