use bevy::prelude::*;

use crate::{
    collisions, common,
    movement::{controls::Controls, dynamics::Dynamics},
};

use super::attack;

#[derive(Component)]
pub struct Player;

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(Player)
        .insert(Name::new("Player"))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            texture: asset_server.load("red.png"),
            ..default()
        })
        .insert(Controls::default())
        .insert(Dynamics {
            linear_acceleration: 400.0,
            idle_breaking: 50.0,
            max_speed: 300.0,
            min_spped: -300.0,
            current_speed: 0.0,
            turning_speed: 5.0,
            current_turning_speed: 0.0,
        })
        .insert(collisions::Collideable)
        .insert(attack::Attacking::new(3_f32, 1_f32))
        .insert(common::size::Size { radius: 50.0 });
}
