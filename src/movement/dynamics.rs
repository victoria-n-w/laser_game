use bevy::prelude::*;

use super::controls::{Controls, Drive, Turn};

#[derive(Component)]
pub struct Dynamics {
    pub linear_acceleration: f32,
    pub idle_breaking: f32,
    pub max_speed: f32,
    pub min_spped: f32,
    pub current_speed: f32,
    pub turning_speed: f32,
    pub current_turning_speed: f32,
}

impl Dynamics {
    fn apply_drive(dynamics: &mut Mut<Self>, controls: &Controls, dtime: f32) {
        match controls.drive {
            Drive::Forwards => {
                let new_speed = dynamics
                    .linear_acceleration
                    .mul_add(dtime, dynamics.current_speed);
                dynamics.current_speed = if new_speed > dynamics.max_speed {
                    dynamics.max_speed
                } else {
                    new_speed
                }
            }
            Drive::Backward => {
                let new_speed = dynamics.current_speed - dynamics.linear_acceleration * dtime;
                dynamics.current_speed = if new_speed < dynamics.min_spped {
                    dynamics.min_spped
                } else {
                    new_speed
                }
            }
            Drive::Idle => {
                if dynamics.current_speed < 0_f32 {
                    let new_speed = dynamics
                        .idle_breaking
                        .mul_add(dtime, dynamics.current_speed);

                    dynamics.current_speed = if new_speed < 0_f32 { new_speed } else { 0_f32 };
                } else if dynamics.current_speed > 0_f32 {
                    let new_speed = dynamics.current_speed - dynamics.idle_breaking * dtime;

                    dynamics.current_speed = if new_speed > 0_f32 { new_speed } else { 0_f32 };
                }
            }
        }
    }

    fn apply_turn(dynamics: &mut Mut<Self>, controls: &Controls) {
        match controls.turn {
            Turn::Anticlockwise => dynamics.current_turning_speed = dynamics.turning_speed,
            Turn::Clockwise => dynamics.current_turning_speed = -1_f32 * dynamics.turning_speed,
            Turn::Idle => dynamics.current_turning_speed = 0_f32,
        };
    }

    pub fn apply(mut dynamics: Mut<Self>, controls: &Controls, dtime: f32) {
        Self::apply_drive(&mut dynamics, controls, dtime);
        Self::apply_turn(&mut dynamics, controls);
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn process_controls(time: Res<Time>, mut entity_query: Query<(&Controls, &mut Dynamics)>) {
    for (controls, dynamics) in entity_query.iter_mut() {
        Dynamics::apply(dynamics, controls, time.delta_seconds());
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn process_moves(time: Res<Time>, mut entities_query: Query<(&Dynamics, &mut Transform)>) {
    entities_query.for_each_mut(|(dynamics, mut transform)| {
        transform.rotate(Quat::from_rotation_z(
            dynamics.current_turning_speed * time.delta_seconds(),
        ));

        let delta_position =
            transform.rotation * (Vec3::X * dynamics.current_speed * time.delta_seconds());

        transform.translation += delta_position;
    });
}
