use self::dynamics::{move_entities, process_controls};

pub mod controls;
pub mod dynamics;

pub struct Plugin;

/// Initializes the systems neccesery to convert controls signals to movement
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(process_controls).add_system(move_entities);
    }
}
