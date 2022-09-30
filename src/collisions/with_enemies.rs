use bevy::prelude::*;

use crate::{enemies, player, states};

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

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn on_collision(
    mut commands: Commands,
    mut collisions: EventReader<Collision>,
    mut publisher: EventWriter<states::TransitionInto>,
    player_state: Query<
        &player::attack::Attacking,
        (
            Without<enemies::entity::Enemy>,
            With<player::entity::Player>,
        ),
    >,
) {
    let attacking = player_state
        .get_single()
        .expect("Could not get a single player");

    for collision in collisions.iter() {
        if attacking.is_active() {
            commands.entity(collision.with).despawn_recursive();
        } else {
            println!("COLLISION - YOU LOOSE HEALTH (to be implemented)");
            publisher.send(states::TransitionInto {
                state: states::AppState::GameOver,
            });
        };
    }
}
