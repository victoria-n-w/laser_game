use std::time::Duration;

use bevy::prelude::*;

use crate::movement::controls::{Controls, Drive, Turn};
use crate::movement::dynamics::Dynamics;

use super::homing::HomingMovement;
use super::random_moves::RandomMoves;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle<T: Component> {
    enemy: Enemy,
    dynamics: Dynamics,
    #[bundle]
    sprite: SpriteBundle,
    controls: Controls,
    movement: T,
}

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(EnemyBundle {
        dynamics: Dynamics {
            linear_acceleration: 400.0,
            idle_breaking: 400.0,
            max_speed: 100.0,
            min_spped: -150.0,
            current_speed: 0.0,
            turning_speed: 10.0,
            current_turning_speed: 0.0,
        },
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
        controls: Controls {
            drive: Drive::Forwards,
            turn: Turn::Idle,
        },
        enemy: Enemy,
        movement: HomingMovement,
    });
}
