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
    dynamics: movement::dynamics::Dynamics,
    collisions: collisions::Collideable,
    attacking: attack::Attacking,
    size: common::size::Size,
}

impl PlayerBundle {
    pub fn new(x: f32, y: f32, rotation: f32, asset_server: &Res<AssetServer>) -> Self {
        PlayerBundle {
            player: Player,
            name: Name::new("PLAYER"),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                texture: asset_server.load("red.png"),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    rotation: Quat::from_rotation_z(rotation),
                    scale: Vec3::new(1.0, 1.0, 1.0),
                },
                ..default()
            },
            collisions: collisions::Collideable,
            controls: movement::controls::Controls::default(),
            attacking: attack::Attacking::new(3_f32, 1_f32),
            size: common::size::Size { radius: 25.0 },
            dynamics: movement::dynamics::Dynamics {
                linear_acceleration: 400.0,
                idle_breaking: 50.0,
                max_speed: 300.0,
                min_spped: -300.0,
                current_speed: 0.0,
                turning_speed: 5.0,
                current_turning_speed: 0.0,
            },
        }
    }
}

#[derive(Component, Default)]
pub struct Fire;

#[derive(Bundle)]
struct FireBundle {
    #[bundle]
    sprite: SpriteSheetBundle,
    animation: animation::Repeating,
    fire: Fire,
}

impl FireBundle {
    pub fn new(
        fps: i32,
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let texture_handle = asset_server.load("fire.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 30, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        FireBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform {
                    translation: Vec3::new(-100.0, 0.0, 0.0),
                    rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                    scale: Vec3::new(4.0, 4.0, 1.0),
                },
                visibility: Visibility { is_visible: false },
                ..default()
            },
            animation: animation::Repeating(Timer::from_seconds(1_f32 / fps as f32, true)),
            fire: Fire,
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player = commands
        .spawn_bundle(PlayerBundle::new(0.0, 0.0, 0.0, &asset_server))
        .id();

    let fire = commands
        .spawn_bundle(FireBundle::new(30, &asset_server, &mut texture_atlases))
        .id();

    commands.entity(player).add_child(fire);
}
