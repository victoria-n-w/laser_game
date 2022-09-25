use bevy::prelude::*;

#[derive(Component)]
pub struct Persisent;

pub fn despawn_all(mut commands: Commands, query: Query<Entity, Without<Persisent>>) {
    query.for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}
