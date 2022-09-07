use bevy::prelude::*;

enum TurnControl {
    Angle(f32),
    Speed(f32),
}

#[derive(Component)]
pub struct SimpleControls {
    pub velocity: f32,
    pub max_angular_velocity: f32,
    turn_control: TurnControl,
}

impl SimpleControls {
    pub const fn new(velocity: f32, max_angular_velocity: f32) -> Self {
        Self {
            velocity,
            max_angular_velocity,
            turn_control: TurnControl::Angle(0_f32),
        }
    }

    /// - angle - angle in radians, ignored if nan
    pub fn set_target_angle(&mut self, angle: f32) {
        if angle.is_nan() {
            return;
        }

        self.turn_control = TurnControl::Angle(angle);
    }

    pub fn set_angular_velocity(&mut self, target: f32) {
        self.turn_control =
            TurnControl::Speed(target.clamp(-self.max_angular_velocity, self.max_angular_velocity));
    }

    pub fn apply_to(&mut self, transform: &mut Transform, dtime_seconds: f32) {
        let d_angle = match self.turn_control {
            TurnControl::Angle(target) => {
                let res = target.clamp(
                    -self.max_angular_velocity * dtime_seconds,
                    self.max_angular_velocity * dtime_seconds,
                );
                self.turn_control = TurnControl::Angle(target - res);
                res
            }
            TurnControl::Speed(speed) => speed * dtime_seconds,
        };

        transform.rotate_z(d_angle);

        transform.translation += transform.local_x() * self.velocity * dtime_seconds;
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn system(time: Res<Time>, mut entities: Query<(&mut SimpleControls, &mut Transform)>) {
    entities.for_each_mut(|(mut controls, mut transform)| {
        controls.apply_to(&mut transform, time.delta_seconds());
    });
}
