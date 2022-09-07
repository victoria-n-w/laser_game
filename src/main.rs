#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::undocumented_unsafe_blocks)]

use bevy::prelude::*;

mod camera;
mod collisions;
mod enemies;
mod math;
mod movement;
mod player;

fn main() {
    App::new()
        .add_plugin(camera::CameraAndWindowPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(player::Plugin)
        .add_plugin(enemies::RandomEnemiesPlugin)
        .add_plugin(movement::Plugin)
        .run();
}
