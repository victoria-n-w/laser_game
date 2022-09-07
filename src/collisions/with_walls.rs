use bevy::prelude::*;

use crate::arena;

use super::Collideable;

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn collision_system(
    arena: Res<arena::Bounds>,
    mut entities: Query<(&mut Transform, &Collideable)>,
) {
    entities.for_each_mut(|(mut transform, _)| {
        transform.translation.x = transform.translation.x.clamp(arena.min_x, arena.max_x);
        transform.translation.y = transform.translation.y.clamp(arena.min_y, arena.max_y);
    });
}
