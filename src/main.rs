#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::undocumented_unsafe_blocks)]

use bevy::prelude::*;

mod camera;
mod enemies;
mod math;
mod movement;
mod player;
use camera::camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)
        .add_plugin(player::Plugin)
        .add_plugin(enemies::RandomEnemiesPlugin)
        .add_plugin(movement::Plugin)
        .run();
}
