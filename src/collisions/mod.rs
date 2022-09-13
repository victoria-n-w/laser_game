mod with_enemies;
mod with_walls;

use bevy::prelude::*;

use crate::{math, AppState};

#[allow(clippy::module_name_repetitions)] // it's used only once and other name would be ambigous
pub struct CollisionsPlugin;

#[derive(Component, Default)]
pub struct Collideable;

impl bevy::prelude::Plugin for CollisionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<with_enemies::Collision>().add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(with_walls::system)
                .with_system(
                    with_enemies::collisions::<math::distance::Manhatann>
                        .before(with_enemies::on_collision),
                )
                .with_system(with_enemies::on_collision),
        );
    }
}
