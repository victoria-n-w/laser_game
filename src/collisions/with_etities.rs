use bevy::prelude::*;

pub trait CollisionEvent: Component {
    fn with_entity(with: Entity) -> Self;
}

use crate::{math, player, util};

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn collisions<M: math::distance::Metric, C: Component, E: CollisionEvent>(
    mut events: EventWriter<E>,
    player_state: Query<
        (&Transform, &util::size::Size),
        (Without<C>, With<player::entity::Player>),
    >,
    entities_locations: Query<(Entity, &Transform, &util::size::Size), With<C>>,
) {
    let (player_transform, player_size) =
        player_state.get_single().expect("Could not get the player");

    let player_location = player_transform.translation;

    entities_locations.for_each(|(id, x, size)| {
        if M::measure(&player_location, &x.translation) < player_size.radius + size.radius {
            events.send(E::with_entity(id));
        };
    });
}
