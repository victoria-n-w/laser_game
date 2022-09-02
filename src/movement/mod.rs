use bevy::prelude::Plugin;

use self::dynamics::{controls_to_dynamics, dynamics_to_transform};

pub mod controls;
pub mod dynamics;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(controls_to_dynamics)
            .add_system(dynamics_to_transform);
    }
}
