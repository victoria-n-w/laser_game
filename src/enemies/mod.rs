use bevy::prelude::{Plugin, SystemSet};

use crate::states::AppState;

pub mod entity;
mod homing;
mod random;
mod spawning;

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(spawning::EggsPlugin::<random::Navigation>::default())
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(random::navigation));
    }
}

pub struct HomingPlugin;

impl Plugin for HomingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(spawning::EggsPlugin::<homing::Navigation>::default())
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(homing::navigation));
    }
}
