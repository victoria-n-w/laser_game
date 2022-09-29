use bevy::prelude::*;

use crate::util;

fn camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(util::commands::Persisent); // do not despawn camera on cleanup
}

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "COLOR GAME".to_string(),
            width: 800.0,
            height: 800.0,
            decorations: false,
            cursor_visible: false,
            scale_factor_override: Some(1.0),
            ..default()
        })
        .add_startup_system(camera);
    }
}
