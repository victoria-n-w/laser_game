use std::marker::PhantomData;

use bevy::{prelude::*, sprite::Sprite, time::Time};
use rand::Rng;

use crate::arena;

use super::enemy_entity;

#[derive(Component)]
struct Egg<T: enemy_entity::Navigation> {
    timer: Timer,
    phantom: PhantomData<T>,
}

#[derive(Bundle)]
pub struct EggBundle<T: enemy_entity::Navigation> {
    #[bundle]
    sprite_bundle: SpriteBundle,
    egg: Egg<T>,
}

impl<T: enemy_entity::Navigation> EggBundle<T> {
    pub fn new(x: f32, y: f32, incubation_time: f32, asset_server: &Res<AssetServer>) -> Self {
        Self {
            egg: Egg::<T> {
                timer: Timer::from_seconds(incubation_time, false),
                phantom: PhantomData,
            },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x, y, 0_f32),
                    rotation: Quat::default(),
                    scale: Vec3::new(1_f32, 1_f32, 1_f32),
                },
                texture: asset_server.load("egg.png"),
                ..default()
            },
        }
    }
}

#[derive(Component)]
pub struct SpawnParametersFor<T: enemy_entity::Navigation> {
    max_n: usize,
    timer: Timer,
    incubation_time: f32,
    phantom: PhantomData<T>,
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
fn spawning_eggs<T: enemy_entity::Navigation>(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    arena_size: Res<arena::Bounds>,
    mut spawn_params: ResMut<SpawnParametersFor<T>>,
    enemies: Query<(&enemy_entity::Enemy, &T)>,
) {
    spawn_params.timer.tick(time.delta());

    if spawn_params.timer.finished() && enemies.into_iter().len() < spawn_params.max_n {
        spawn_params.max_n += 1;

        let mut rng = rand::thread_rng();

        let x = rng.gen_range(arena_size.min_x..arena_size.max_x);
        let y = rng.gen_range(arena_size.min_y..arena_size.max_y);

        commands.spawn_bundle(EggBundle::<T>::new(
            x,
            y,
            spawn_params.incubation_time,
            &asset_server,
        ));
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
fn hatchin_eggs<T: enemy_entity::Navigation>(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut eggs: Query<(Entity, &mut Egg<T>, &Transform)>,
) {
    eggs.for_each_mut(|(id, mut egg, transform)| {
        egg.timer.tick(time.delta());

        if egg.timer.finished() {
            commands.entity(id).despawn_recursive();
            commands.spawn_bundle(enemy_entity::EnemyBundle::<T>::new(
                transform.translation.x,
                transform.translation.y,
                &asset_server,
            ));
        };
    });
}

#[derive(Default)]
pub struct EggsPlugin<T: enemy_entity::Navigation> {
    phantom: PhantomData<T>,
}

impl<T: enemy_entity::Navigation> Plugin for EggsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnParametersFor::<T> {
            max_n: 10,
            timer: Timer::from_seconds(7_f32, true),
            incubation_time: 4_f32,
            phantom: PhantomData,
        })
        .add_system(spawning_eggs::<T>)
        .add_system(hatchin_eggs::<T>);
    }
}
