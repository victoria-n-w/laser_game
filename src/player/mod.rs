use bevy::prelude::Plugin;

mod controls;
mod player;

use self::controls::player_controls;
use self::player::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player)
            .add_system(player_controls);
    }
}
