mod with_enemies;
mod with_walls;

use bevy::prelude::Component;

use crate::math;

pub struct Plugin;

#[derive(Component, Default)]
pub struct Collideable;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(with_walls::system)
            .add_event::<with_enemies::Collision>()
            .add_system(with_enemies::collisions::<math::distance::Manhatann>)
            .add_system(with_enemies::on_collision);
    }
}
