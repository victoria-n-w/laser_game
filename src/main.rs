use bevy::prelude::*;

mod camera;
mod enemies;
mod movement;
mod player;
use camera::camera_system;
use enemies::RandomEnemiesPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_system)
        .add_plugin(PlayerPlugin)
        .add_plugin(RandomEnemiesPlugin)
        .add_plugin(MovementPlugin)
        .run();
}
