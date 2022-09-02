use bevy::prelude::Plugin;

mod controls;
pub mod entity;

use self::controls::player_controls;
use self::entity::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player)
            .add_system(player_controls);
    }
}
