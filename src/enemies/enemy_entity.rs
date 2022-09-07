use bevy::prelude::*;

use crate::movement;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle<T: Component> {
    enemy: Enemy,
    #[bundle]
    sprite: SpriteBundle,
    navigation: T,
    movement: movement::simple_moves::SimpleControls,
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn spawn_enemy<T: Default + Component>(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(EnemyBundle {
        sprite: SpriteBundle {
            transform: Transform {
                translation: default(),
                rotation: default(),
                scale: Vec3 {
                    x: 0.09,
                    y: 0.09,
                    z: 1.0,
                },
            },
            texture: asset_server.load("blue.png"),
            ..default()
        },
        enemy: Enemy,
        navigation: T::default(),
        movement: movement::simple_moves::SimpleControls::new(150.0, 5.0),
    });
}
