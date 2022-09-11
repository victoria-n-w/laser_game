pub mod attack;
mod controls;
pub mod entity;

use self::controls::process_keyboard_input;
use self::entity::spawn_player;

/// Plugin responsible for initializing the player and converting keyboard events to control signals
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player)
            .add_system(process_keyboard_input)
            .add_event::<attack::Started>()
            .add_event::<attack::Finished>()
            .add_system(attack::system)
            .add_system(attack::animate_start)
            .add_system(attack::animate_end);
    }
}
