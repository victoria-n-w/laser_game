mod with_enemies;
mod with_walls;

use bevy::prelude::*;

use crate::math;

#[allow(clippy::module_name_repetitions)] // it's used only once and other name would be ambigous
pub struct CollisionsPlugin;

#[derive(Component, Default)]
pub struct Collideable;

impl bevy::prelude::Plugin for CollisionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(with_walls::system)
            .add_event::<with_enemies::Collision>()
            .add_system(
                with_enemies::collisions::<math::distance::Manhatann>
                    .before(with_enemies::on_collision),
            )
            .add_system(with_enemies::on_collision);
    }
}
