use bevy::prelude::*;

use crate::{collisions, movement};

#[derive(Component)]
pub struct Enemy;

pub trait Navigation: Component + Default {
    fn texture_path() -> String;
}

#[derive(bevy::prelude::Bundle)]
pub struct EnemyBundle<T: Navigation> {
    enemy: Enemy,
    #[bundle]
    sprite: SpriteBundle,
    navigation: T,
    movement: movement::simple_moves::SimpleControls,
    collisions: collisions::Collideable,
}

impl<T: Navigation> EnemyBundle<T> {
    pub fn new(x: f32, y: f32, asset_server: &Res<AssetServer>) -> Self {
        Self {
            enemy: Enemy,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                texture: asset_server.load(&T::texture_path()),
                transform: Transform {
                    translation: Vec3::new(x, y, 0_f32),
                    rotation: Quat::default(),
                    scale: Vec3::new(1_f32, 1_f32, 1_f32),
                },
                ..default()
            },
            navigation: T::default(),
            movement: movement::simple_moves::SimpleControls::new(150.0, 5.0),
            collisions: collisions::Collideable,
        }
    }
}
