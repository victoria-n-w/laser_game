use bevy::prelude::SystemSet;

use crate::AppState;

pub mod controls;
pub mod dynamics;
pub mod simple_moves;

pub struct Plugin;

/// Initializes the systems neccesery to convert controls signals to movement
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(dynamics::process_controls)
                .with_system(dynamics::process_moves)
                .with_system(simple_moves::system),
        );
    }
}
