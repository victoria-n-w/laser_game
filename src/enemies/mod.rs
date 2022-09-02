use bevy::prelude::Plugin;

use self::{enemy_entity::spawn_enemy, random_moves::control_random_moves};

mod enemy_entity;
mod homing;
mod random_moves;

pub struct RandomEnemiesPlugin;

impl Plugin for RandomEnemiesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_enemy)
            .add_system(control_random_moves);
    }
}
