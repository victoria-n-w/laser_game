use bevy::prelude::*;

use rand::Rng;

use crate::{arena, collisions, states, trackers, util};

#[derive(Component)]
pub struct ScorePickup;

#[derive(Bundle)]
struct PickupBundle {
    pickup: ScorePickup,
    #[bundle]
    sprite: SpriteBundle,
    collisions: collisions::Collideable,
    size: util::size::Size,
}

impl PickupBundle {
    pub fn new(x: f32, y: f32, asset_server: &Res<AssetServer>) -> Self {
        Self {
            pickup: ScorePickup,
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                texture: asset_server.load("star.png"),
                transform: Transform {
                    translation: Vec3::new(x, y, 0_f32),
                    rotation: Quat::default(),
                    scale: Vec3::new(1_f32, 1_f32, 1_f32),
                },
                ..default()
            },
            collisions: collisions::Collideable,
            size: util::size::Size { radius: 25_f32 },
        }
    }
}

#[derive(Component)]
pub struct PickedUp {
    id: Entity,
}

fn spawn_random_pickup(
    commands: &mut Commands,
    arena_size: &Res<arena::Bounds>,
    asset_server: &Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(arena_size.min_x..arena_size.max_x);
    let y = rng.gen_range(arena_size.min_y..arena_size.max_y);
    commands.spawn_bundle(PickupBundle::new(x, y, asset_server));
}

fn picked_up(
    mut commands: Commands,
    arena_size: Res<arena::Bounds>,
    asset_server: Res<AssetServer>,
    mut events: EventReader<PickedUp>,
    mut sender: EventWriter<trackers::score::Change>,
) {
    for event in events.iter() {
        sender.send(trackers::score::Change { value: 10 });
        commands.entity(event.id).despawn_recursive();
        spawn_random_pickup(&mut commands, &arena_size, &asset_server);
    }
}

fn setup(mut commands: Commands, arena_size: Res<arena::Bounds>, asset_server: Res<AssetServer>) {
    spawn_random_pickup(&mut commands, &arena_size, &asset_server);
}

pub struct PickupsPlugin;

impl Plugin for PickupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickedUp>()
            .add_system_set(SystemSet::on_enter(states::AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_update(states::AppState::Game).with_system(picked_up));
    }
}
