use bevy::prelude::*;

use crate::movement::{
    controls::{Controls, Drive, Turn},
    dynamics::Dynamics,
};

#[derive(Component)]
pub struct Player;

pub(crate) fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    x: 0.03,
                    y: 0.03,
                    z: 1.0,
                },
            },
            ..default()
        })
        .insert(Controls {
            drive: Drive::Idle,
            turn: Turn::Idle,
        })
        .insert(Dynamics {
            linear_acceleration: 400.0,
            idle_breaking: 350.0,
            max_speed: 200.0,
            min_spped: -200.0,
            current_speed: 0.0,
            turning_speed: 5.0,
            current_turning_speed: 0.0,
        });
}