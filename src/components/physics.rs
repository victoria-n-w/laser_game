use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct Acceleration {
    mass: f32,
    max_speed: f32,
}
