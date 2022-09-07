pub mod controls;
pub mod dynamics;
pub mod simple_moves;

pub struct Plugin;

/// Initializes the systems neccesery to convert controls signals to movement
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(dynamics::process_controls)
            .add_system(dynamics::process_moves)
            .add_system(simple_moves::system);
    }
}
