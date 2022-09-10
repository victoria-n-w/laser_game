use bevy::prelude::*;

#[derive(Component)]
pub struct Collision {
    pub with: Entity,
}

use crate::{common, enemies, math, player};

pub fn collisions<M: math::distance::Metric>(
    mut events: EventWriter<Collision>,
    player_state: Query<
        (&Transform, &common::size::Size),
        (
            Without<enemies::entity::Enemy>,
            With<player::entity::Player>,
        ),
    >,
    enemies_locations: Query<
        (Entity, &Transform, &common::size::Size),
        With<enemies::entity::Enemy>,
    >,
) {
    let (player_transform, player_size) =
        player_state.get_single().expect("Could not get the player");

    let player_location = player_transform.translation;

    enemies_locations.for_each(|(id, x, enemy_size)| {
        if M::measure(&player_location, &x.translation) < player_size.radius + enemy_size.radius {
            events.send(Collision { with: id });
        };
    });
}

pub fn on_collision(
    mut events: EventReader<Collision>,
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

    // mock for testing only
    for ev in events.iter() {
        println!(
            "Collision with: {} while: {}",
            ev.with.id(),
            attacking.is_active()
        );
    }
}