#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::type_complexity)] // types in some queries can get quite complex

use bevy::prelude::*;

mod animation;
mod arena;
mod camera;
mod collisions;
mod enemies;
mod math;
mod movement;
mod pickups;
mod player;
mod screens;
mod states;
mod trackers;
mod util;

fn main() {
    App::new()
        .add_state(states::AppState::Title)
        .add_plugin(arena::Plugin {
            x: 800_f32,
            y: 800_f32,
        })
        .add_plugin(states::StatesPlugin)
        .add_plugin(camera::RenderPlugin)
        .add_plugin(screens::Plugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(player::Plugin)
        .add_plugin(enemies::RandomPlugin)
        .add_plugin(enemies::HomingPlugin)
        .add_plugin(collisions::CollisionsPlugin)
        .add_plugin(movement::Plugin)
        .add_plugin(trackers::TrackersPlugin)
        .add_plugin(pickups::PickupsPlugin)
        .add_system(animation::animate_sprites)
        .run();
}
