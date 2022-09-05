mod controls;
pub mod entity;

use self::controls::process_keyboard_input;
use self::entity::spawn_player;

/// Plugin responsible for initializing the player and converting keyboard events to control signals
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player)
            .add_system(process_keyboard_input);
    }
}
