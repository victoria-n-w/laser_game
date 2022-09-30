use bevy::prelude::*;

use crate::states;

#[derive(Component)]
struct HealthConfig {
    starting_health: i32,
    /// height of a single heart container
    height: f32,
    /// width of a single heart container
    width: f32,
    /// where to start drawing the health bar
    top_left: (f32, f32),
    hearts: Vec<Entity>,
}

#[derive(Component)]
pub struct Damage;

fn damage(
    mut commands: Commands,
    mut health: ResMut<HealthConfig>,
    mut stage_change: EventWriter<states::TransitionInto>,
    mut damage_events: EventReader<Damage>,
) {
    for _ in damage_events.iter() {
        match health.hearts.pop() {
            Some(id) => {
                commands.entity(id).despawn_recursive();
            }
            None => stage_change.send(states::TransitionInto {
                state: states::AppState::GameOver,
            }),
        }
    }
}

fn new_heart(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    asset_server: &Res<AssetServer>,
) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        texture: asset_server.load("love-shield.png"),
        transform: Transform {
            translation: Vec3::new(x, y, -0_f32),
            rotation: Quat::default(),
            scale: Vec3::new(1_f32, 1_f32, 1_f32),
        },
        ..default()
    }
}

fn setup(mut commands: Commands, mut health: ResMut<HealthConfig>, asset_server: Res<AssetServer>) {
    let (x, y) = health.top_left;

    let starting_x = x + health.width / 2.0;
    let starting_y = y - health.height / 2.0;

    for i in 0..health.starting_health {
        let id = commands
            .spawn_bundle(new_heart(
                i as f32 * health.width + starting_x,
                starting_y,
                health.width,
                health.height,
                &asset_server,
            ))
            .id();
        health.hearts.push(id);
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HealthConfig {
            starting_health: 3,
            height: 50.0,
            width: 50.0,
            top_left: (-400.0, 400.0),
            hearts: Vec::new(),
        })
        .add_event::<Damage>()
        .add_system_set(SystemSet::on_enter(states::AppState::Game).with_system(setup))
        .add_system_set(
            SystemSet::on_update(states::AppState::Game)
                .with_system(damage.before(states::transitioning)),
        );
    }
}
