use bevy::prelude::Plugin;

use self::{
    enemy_entity::spawn_enemy, homing::move_homing_enemies, random_moves::generate_random_controls,
};

mod enemy_entity;
mod homing;
mod random_moves;

pub struct RandomEnemiesPlugin;

impl Plugin for RandomEnemiesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_enemy)
            .add_system(generate_random_controls)
            .add_system(move_homing_enemies);
    }
}
