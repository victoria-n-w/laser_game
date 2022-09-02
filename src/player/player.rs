use bevy::prelude::*;

use super::controls::controls_system;

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(Player)
        .insert(Name::new("Player"))
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("red.png"),
            transform: Transform {
                translation: default(),
                rotation: default(),
                scale: Vec3 {
                    x: 0.1,
                    y: 0.1,
                    z: 1.0,
                },
            },
            ..default()
        });
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_player)
            .add_system(controls_system);
    }
}
