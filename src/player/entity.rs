use bevy::prelude::*;

use crate::{animation, collisions, common, movement};

use super::attack;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    name: Name,
    #[bundle]
    sprite: SpriteBundle,
    controls: movement::controls::Controls,
    collisions: collisions::Collideable,
    attacking: attack::Attacking,
    size: common::size::Size,
}

#[derive(Component)]
pub struct Fire;

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player = commands
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
        .insert(movement::controls::Controls::default())
        .insert(movement::dynamics::Dynamics {
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
        .insert(common::size::Size { radius: 25.0 })
        .id();

    let texture_handle = asset_server.load("fire.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 30, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let fire = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: Vec3::new(-100.0, 0.0, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                scale: Vec3::new(4.0, 4.0, 1.0),
            },
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(animation::Repeating(Timer::from_seconds(
            1_f32 / 30_f32,
            true,
        )))
        .insert(Fire)
        .id();

    commands.entity(player).add_child(fire);
}
