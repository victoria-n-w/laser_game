use bevy::{ecs::system::Command, prelude::*};

#[derive(Component)]
pub struct Persisent;

pub struct DespawnAll;

impl Command for DespawnAll {
    fn write(self, world: &mut World) {
        let entities = world
            .query_filtered::<Entity, Without<Persisent>>()
            .iter(world)
            .collect::<Vec<Entity>>();

        for e in entities {
            world.entity_mut(e).despawn_recursive();
        }
    }
}
