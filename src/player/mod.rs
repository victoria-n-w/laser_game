pub mod attack;
mod controls;
pub mod entity;

use bevy::prelude::SystemSet;

use crate::states::AppState;

use self::controls::process_keyboard_input;
use self::entity::spawn;

/// Plugin responsible for initializing the player and converting keyboard events to control signals
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<attack::Started>()
            .add_event::<attack::Finished>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(entity::despawn))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(process_keyboard_input)
                    .with_system(attack::system)
                    .with_system(attack::animate_start)
                    .with_system(attack::animate_end),
            );
    }
}
