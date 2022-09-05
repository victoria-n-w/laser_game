use bevy::prelude::{Camera2dBundle, Commands};

pub fn camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
