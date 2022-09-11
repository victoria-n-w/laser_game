use std::time::Duration;

use bevy::prelude::*;

use super::entity;

enum State {
    OnCooldown(Timer),
    Active(Timer),
}

pub enum Change {
    NoChange,
    JustFinished,
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
        Self {
            cooldown_time: cooldown,
            duration_time: duration,
            ..default()
        }
    }

    pub fn trigger(&mut self) -> bool {
        match &self.state {
            State::OnCooldown(timer) => {
                if timer.finished() {
                    self.state = State::Active(Timer::from_seconds(self.duration_time, false));
                    true
                } else {
                    false
                }
            }
            State::Active(_) => false,
        }
    }

    pub fn pass_time(&mut self, dtime: Duration) -> Change {
        match &mut self.state {
            State::OnCooldown(timer) => {
                timer.tick(dtime);
                Change::NoChange
            }
            State::Active(timer) => {
                timer.tick(dtime);
                if timer.finished() {
                    self.state = State::OnCooldown(Timer::from_seconds(self.cooldown_time, false));
                    Change::JustFinished
                } else {
                    Change::NoChange
                }
            }
        }
    }

    pub const fn is_active(&self) -> bool {
        match self.state {
            State::OnCooldown(_) => false,
            State::Active(_) => true,
        }
    }
}

#[derive(Default)]
pub struct Started;

#[derive(Default)]
pub struct Finished;

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn system(
    time: Res<Time>,
    mut attack_finished: EventWriter<Finished>,
    mut entities: Query<&mut Attacking>,
) {
    entities.for_each_mut(|mut attacking| match attacking.pass_time(time.delta()) {
        Change::NoChange => {}
        Change::JustFinished => {
            attack_finished.send_default();
        }
    });
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn animate_start(
    events: EventReader<Started>,
    mut fire: Query<&mut Visibility, With<entity::Fire>>,
) {
    if !events.is_empty() {
        let mut visible = fire
            .get_single_mut()
            .expect("Could not get a single player");

        visible.is_visible = true;
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn animate_end(
    events: EventReader<Finished>,
    mut fire: Query<&mut Visibility, With<entity::Fire>>,
) {
    if !events.is_empty() {
        let mut visible = fire
            .get_single_mut()
            .expect("Could not get a single player");

        visible.is_visible = false;
    }
}
