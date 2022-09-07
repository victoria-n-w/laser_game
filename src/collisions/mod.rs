mod with_walls;

use bevy::prelude::Component;

pub struct Plugin;

#[derive(Component, Default)]
pub struct Collideable;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(with_walls::collision_system);
    }
}
