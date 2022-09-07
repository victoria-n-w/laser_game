use bevy::prelude::Plugin;

use self::{
    enemy_entity::spawn_enemy, homing::move_homing_enemies, random::generate_random_controls,
};

mod enemy_entity;
mod homing;
mod random;

pub struct RandomEnemiesPlugin;

impl Plugin for RandomEnemiesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_enemy::<homing::Navigation>)
            .add_startup_system(spawn_enemy::<random::Navigation>)
            .add_system(generate_random_controls)
            .add_system(move_homing_enemies);
    }
}
