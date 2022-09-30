use bevy::prelude::*;

use crate::{arena, pickups, trackers};

use super::with_etities;

#[derive(Component)]
pub struct PickedUp {
    id: Entity,
}

impl with_etities::CollisionEvent for PickedUp {
    fn with_entity(with: Entity) -> Self {
        Self { id: with }
    }
}

pub fn picked_up(
    mut commands: Commands,
    arena_size: Res<arena::Bounds>,
    asset_server: Res<AssetServer>,
    mut events: EventReader<PickedUp>,
    mut sender: EventWriter<trackers::score::Change>,
) {
    for event in events.iter() {
        sender.send(trackers::score::Change { value: 10 });
        commands.entity(event.id).despawn_recursive();
        pickups::spawn_random_pickup(&mut commands, &arena_size, &asset_server);
    }
}
