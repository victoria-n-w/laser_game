use bevy::prelude::*;

mod camera;
mod movement;
mod player;
use camera::camera_system;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_system)
        .add_plugin(PlayerPlugin)
        .add_plugin(MovementPlugin)
        .run();
}
