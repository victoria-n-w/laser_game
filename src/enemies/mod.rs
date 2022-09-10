use bevy::prelude::Plugin;

pub mod entity;
mod homing;
mod random;
mod spawning;

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(spawning::EggsPlugin::<random::Navigation>::default())
            .add_system(random::navigation_system);
    }
}

pub struct HomingPlugin;

impl Plugin for HomingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(spawning::EggsPlugin::<homing::Navigation>::default())
            .add_system(homing::navigation_system);
    }
}
