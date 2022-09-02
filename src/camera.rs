use bevy::prelude::{Camera2dBundle, Commands};

pub fn camera_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
