use bevy::prelude::*;

use crate::{enemies, player, trackers};

use super::with_etities;

#[derive(Component)]
pub struct Collision {
    pub with: Entity,
}

impl with_etities::CollisionEvent for Collision {
    fn with_entity(with: Entity) -> Self {
        Self { with }
    }
}

#[derive(Component)]
pub struct InvAfterDamage {
    pub timer: Timer,
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn on_collision(
    mut commands: Commands,
    time: Res<Time>,
    mut inv_period: ResMut<InvAfterDamage>,
    mut collisions: EventReader<Collision>,
    mut publisher: EventWriter<trackers::health::Damage>,
    player_state: Query<
        &player::attack::Attacking,
        (
            Without<enemies::entity::Enemy>,
            With<player::entity::Player>,
        ),
    >,
) {
    inv_period.timer.tick(time.delta());

    let attacking = player_state
        .get_single()
        .expect("Could not get a single player");

    for collision in collisions.iter() {
        if attacking.is_active() {
            commands.entity(collision.with).despawn_recursive();
        } else {
            if inv_period.timer.finished() {
                inv_period.timer.reset();
                publisher.send(trackers::health::Damage);
            }
        };
    }
}
