use bevy::prelude::*;

mod components;

mod camera;
mod player;
use camera::camera_system;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_system)
        .add_plugin(PlayerPlugin)
        .run();
}
